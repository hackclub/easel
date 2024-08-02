mod display;

use std::{
    cmp::Ordering,
    collections::HashMap,
    ffi::{c_char, c_int, c_void, CStr},
    fmt,
    io::{Read, Write},
    str::FromStr,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

use async_std::{
    channel::{self, Receiver, Sender, TryRecvError},
    io,
    sync::RwLock,
};
use bitflags::bitflags;
use clap::Args;
use clio::Input;
use display::TextBuffer;
use libloading::Library;
use modular_bitfield::prelude::*;
use thiserror::Error;

const CTRL_LOW: &[u8; 1 << 8] = include_bytes!(concat!(env!("OUT_DIR"), "/ctrl_low.rom"));
const CTRL_MID: &[u8; 1 << 8] = include_bytes!(concat!(env!("OUT_DIR"), "/ctrl_mid.rom"));
const CTRL_HIGH: &[u8; 1 << 8] = include_bytes!(concat!(env!("OUT_DIR"), "/ctrl_high.rom"));

type NameFn = unsafe extern "C" fn() -> *const c_char;
type InitFn = unsafe extern "C" fn(u8) -> c_int;
type StatefulInitFn = unsafe extern "C" fn(u8) -> *mut c_void;
type ReadFn = unsafe extern "C" fn(u8) -> u8;
type StatefulReadFn = unsafe extern "C" fn(*mut c_void, u8) -> u8;
type WriteFn = unsafe extern "C" fn(u8, u8);
type StatefulWriteFn = unsafe extern "C" fn(*mut c_void, u8, u8);
type TickFn = unsafe extern "C" fn();
type StatefulTickFn = unsafe extern "C" fn(*mut c_void);
type DropFn = unsafe extern "C" fn();
type StatefulDropFn = unsafe extern "C" fn(*mut c_void);
type ResetFn = unsafe extern "C" fn();
type StatefulResetFn = unsafe extern "C" fn(*mut c_void);
type LastErrLen = unsafe extern "C" fn() -> c_int;
type GetLastErr = unsafe extern "C" fn(*mut c_char, c_int) -> c_int;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("unable to read provided input")]
    Input(std::io::Error),
    #[error("error in stdin channel")]
    StdIn,
    #[error("unable to print to stdout")]
    StdOut(std::io::Error),
    #[error("global state already set")]
    OnceFull,
    #[error("global state not initialized yet")]
    OnceEmpty,
}

#[derive(Debug, Args)]
pub struct EmulatorArgs {
    /// Input program ROM
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

enum Command {
    Get,
    Set,
    Peek,
    Poke,
    Drop,
    Run,
    Load,
    Dump,
    Quit,
    Step,
    Reset,
    Help,
    Stop,
    Blank,
}

impl Command {
    fn help(&self) -> &'static str {
        match self {
            Command::Get => "\
                GET <register>:\n\
                \n\
                Gets the current state of the selected register.\n\
                \n\
                `register` must be in the range 0 through 8 (exclusive).\n\
            ",
            Command::Set => "\
                SET <register> <value>:\n\
                \n\
                Sets the value in the selected register to `value`.\n\
                \n\
                `register` must be in the range 0 through 7 (inclusive).\n\
                `value` must be in the range 0 through 255 (inclusive).\n\
            ",
            Command::Peek => "\
                PEEK <address>:\n\
                \n\
                Gets the current value at the selected memory address.
                \n\
                `address` must be in the range 0x0000 through 0xFFBF (inclusive).\n\
            ",
            Command::Poke => "\
                POKE <address> <value>:\n\
                \n\
                Writes the given value to the selected memory address.
                \n\
                `address` must be in the range 0x0000 through 0xFFBF (inclusive).\n\
                `value` must be in the range 0 through 255 (inclusive).\n\
            ",
            Command::Drop => "\
                DROP [port...]:\n\
                \n\
                If 1 or more ports are provided, each port is disconnected from the corresponding peripheral (if one is connected).\n\
                If all ports connected to a peripheral are disconnected, the peripheral is dropped.\n\
                If no ports are provided, all ports are disconnected, dropping all peripherals.\n\
            ",
            Command::Run => "\
                RUN <speed>:\n\
                \n\
                Runs the CPU at the given speed.\n\
                If no speed is supplied, or if the given speed is `0`,\n\
                the CPU will run at maximum speed.\n\
                \n\
                `speed` is measured in hz.\n\
            ",
            Command::Load => "\
                LOAD <module> [port...]:\n\
                \n\
                Loads the given module as a peripheral, connecting it to the given ports.\n\
                If no ports are supplied, the module will not be initialized.\n\
                Read more about peripheral modules in the README\n\
                \n\
                `module` must be a valid path to a shared library.\n\
                Each `port` must be within the range 0xFFC0 through 0xFFFE (inclusive).\n\
            ",
            Command::Dump => "\
                DUMP:\n\
                \n\
                Prints the current machine state, including the:\n\
                - Program Counter\n\
                - Stack Pointer\n\
                - Bus\n\
                - Register Bank\n\
                - Control Word\n\
                - Instruction Register\n\
                - Attached Peripherals\n\
            ",
            Command::Quit => "\
                QUIT:\n\
                \n\
                Exits the emulator after cleaning up.\n\
            ",
            Command::Step => "\
                STEP:\n\
                \n\
                Steps the CPU clock a single time.\n\
                Cannot be used while the CPU is running.\n\
            ",
            Command::Reset => "\
                RESET:\n\
                \n\
                Resets the CPU to it's initial state.\n\
                While `reset` or `stateful_reset` is called on all peripherals,\n\
                no guarantees can be made that all peripherals will be reset to their initial state.\n\
            ",
            Command::Help => "\
                HELP [command]:\n\
                \n\
                With no given command, prints a short help message for each command.\n\
                With a selected command, prints a more detailed help message for the selected command.\n\
            ",
            Command::Stop => "\
                STOP:\n\
                \n\
                Stops the CPU clock.\n\
                Enables the use of `STEP`.\n\
            ",
            Command::Blank => unreachable!(),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Command::Get),
            "SET" => Ok(Command::Set),
            "PEEK" => Ok(Command::Peek),
            "POKE" => Ok(Command::Poke),
            "DROP" => Ok(Command::Drop),
            "RUN" => Ok(Command::Run),
            "LOAD" => Ok(Command::Load),
            "DUMP" => Ok(Command::Dump),
            "QUIT" => Ok(Command::Quit),
            "STEP" => Ok(Command::Step),
            "RESET" => Ok(Command::Reset),
            "HELP" => Ok(Command::Help),
            "STOP" => Ok(Command::Stop),
            "" => Ok(Command::Blank),
            _ => Err(()),
        }
    }
}

bitflags! {
    /// Representation of the CPU Control Word
    ///
    /// Find more in-depth explanations of flags in `Arch.md`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct ControlWord: u32 {
        /// ALU opcode low
        const AOL = 1 << 0;
        /// ALU opcode middle
        const AOM = 1 << 1;
        /// ALU opcode high
        const AOH = 1 << 2;
        /// Arithmetic Operation
        const AO = 1 << 3;
        /// Register Bank In
        const RBI = 1 << 4;
        /// Register Bank Out
        const RBO = 1 << 5;
        /// Register Select Built-in
        const RSB = 1 << 6;
        /// Register Select Primary
        const RSP = 1 << 7;
        /// Stack Pointer Increment
        const SPI = 1 << 8;
        /// Stack Pointer Decrement
        const SPD = 1 << 9;
        /// Clock Reset
        const CR = 1 << 10;
        /// Program Counter Increment
        const PCI = 1 << 11;
        /// Jump if Not Zero
        const JNZ = 1 << 12;
        /// Load Instruction
        const LI = 1 << 13;
        /// Program Out
        const PO = 1 << 14;
        /// Store Register
        const SR = 1 << 15;
        /// Transfer HL
        const THL = 1 << 16;
        /// Load Address
        const LA = 1 << 17;
        /// Store Address
        const SA = 1 << 18;
        /// Address Low In
        const ALI = 1 << 19;
        /// Address High In
        const AHI = 1 << 20;
        /// Load Stack Pointer
        const LSP = 1 << 21;
        /// Load Program Memory
        const LPM = 1 << 22;
        /// Set Halt
        const SH = 1 << 23;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct SReg: u8 {
        /// Zero
        const Z = 1 << 0;
        /// Carry
        const C = 1 << 1;
        /// Less-than
        const L = 1 << 2;
        /// Equal
        const E = 1 << 3;
        /// Greater-than
        const G = 1 << 4;
        /// Halt
        const H = 1 << 7;
    }
}

trait Notified {
    fn notified_add(self, other: Self, sreg: &mut SReg, remove: bool) -> Self;
    fn notified_sub(self, other: Self, sreg: &mut SReg, remove: bool) -> Self;
}

impl Notified for u8 {
    fn notified_add(self, other: Self, sreg: &mut SReg, remove: bool) -> Self {
        match self.checked_add(other) {
            Some(val) => {
                if remove {
                    sreg.remove(SReg::C);
                }
                val
            }
            None => {
                sreg.insert(SReg::C);
                self.wrapping_add(other)
            }
        }
    }

    fn notified_sub(self, other: Self, sreg: &mut SReg, remove: bool) -> Self {
        match self.checked_sub(other) {
            Some(val) => {
                if remove {
                    sreg.remove(SReg::C);
                }
                val
            },
            None => {
                sreg.insert(SReg::C);
                self.wrapping_sub(other)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Alu {
    primary: u8,
    secondary: u8,
}

impl Alu {
    fn compute(&self, aol: bool, aom: bool, aoh: bool, sreg: &mut SReg) -> u8 {
        match (aoh, aom, aol) {
            (false, false, false) => self.primary.notified_add(self.secondary, sreg, true),
            (false, false, true) => self.primary.notified_sub(self.secondary, sreg, true),
            (false, true, false) => self
                .primary
                .notified_add(sreg.contains(SReg::C) as u8, sreg, true)
                .notified_add(self.secondary, sreg, false),
            (false, true, true) => self
                .primary
                .notified_sub(sreg.contains(SReg::C) as u8, sreg, true)
                .notified_sub(self.secondary, sreg, false),
            (true, false, false) => !(self.primary & self.secondary),
            (true, false, true) => self.primary | self.secondary,
            _ => 0x00,
        }
    }

    fn execute(&mut self, sreg: &mut SReg, bus: u8, aol: bool, aom: bool, aoh: bool) {
        match (aoh, aom, aol) {
            (false, false, true) => match self.primary.cmp(&self.secondary) {
                Ordering::Less => {
                    sreg.insert(SReg::L);
                    sreg.remove(SReg::E);
                    sreg.remove(SReg::G);
                }
                Ordering::Equal => {
                    sreg.remove(SReg::L);
                    sreg.insert(SReg::E);
                    sreg.remove(SReg::G);
                }
                Ordering::Greater => {
                    sreg.remove(SReg::L);
                    sreg.remove(SReg::E);
                    sreg.insert(SReg::G);
                }
            },
            (false, true, false) => sreg.set(SReg::Z, self.primary == 0),
            (false, true, true) => self.primary = bus,
            (true, false, false) => self.secondary = bus,
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.primary = 0;
        self.secondary = 0;
    }
}

impl Default for Alu {
    fn default() -> Self {
        Alu {
            primary: 0,
            secondary: 0,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Flags: u8 {
        /// Zero flag
        const Z = 1 << 0;
        /// Carry flag
        const C = 1 << 1;
        /// Less than flag
        const L = 1 << 2;
        /// Equal flag
        const E = 1 << 3;
        /// Greater than flag
        const G = 1 << 4;
        /// Halt flag
        const H = 1 << 7;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegBank {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl RegBank {
    fn get_reg(&self, n: u8) -> u8 {
        match n {
            0 => self.a,
            1 => self.b,
            2 => self.c,
            3 => self.d,
            4 => self.e,
            5 => self.f,
            6 => self.h,
            7 => self.l,
            _ => unreachable!(),
        }
    }

    fn set_reg(&mut self, n: u8, val: u8) {
        match n {
            0 => self.a = val,
            1 => self.b = val,
            2 => self.c = val,
            3 => self.d = val,
            4 => self.e = val,
            5 => self.f = val,
            6 => self.h = val,
            7 => self.l = val,
            _ => unreachable!(),
        }
    }

    fn clear(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.f = 0;
        self.h = 0;
        self.l = 0;
    }
}

impl Default for RegBank {
    fn default() -> Self {
        RegBank {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }
}

impl fmt::Display for RegBank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
                REGISTER A: {:#04X}\n\
                REGISTER B: {:#04X}\n\
                REGISTER C: {:#04X}\n\
                REGISTER D: {:#04X}\n\
                REGISTER E: {:#04X}\n\
                REGISTER F: {:#04X}\n\
                REGISTER H: {:#04X}\n\
                REGISTER L: {:#04X}\n\
            ",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 4]
enum Instruction {
    Add = 0x0,
    Sub = 0x1,
    Adc = 0x2,
    Sbb = 0x3,
    Nand = 0x4,
    Or = 0x5,
    Cmp = 0x6,
    Mv = 0x7,
    Ld = 0x8,
    St = 0x9,
    Lda = 0xA,
    Lpm = 0xB,
    Push = 0xC,
    Pop = 0xD,
    Jnz = 0xE,
    Halt = 0xF,
}

impl From<u8> for Instruction {
    fn from(val: u8) -> Self {
        match val {
            0x0 => Instruction::Add,
            0x1 => Instruction::Sub,
            0x2 => Instruction::Adc,
            0x3 => Instruction::Sbb,
            0x4 => Instruction::Nand,
            0x5 => Instruction::Or,
            0x6 => Instruction::Cmp,
            0x7 => Instruction::Mv,
            0x8 => Instruction::Ld,
            0x9 => Instruction::St,
            0xA => Instruction::Lda,
            0xB => Instruction::Lpm,
            0xC => Instruction::Push,
            0xD => Instruction::Pop,
            0xE => Instruction::Jnz,
            0xF => Instruction::Halt,
            _ => unreachable!(),
        }
    }
}

use __head::InstructionHeader;

/// Seperated into a seperate module to get rid of
/// the dead code warning that was driving me crazy.
#[allow(dead_code)]
mod __head {
    use super::*;

    #[bitfield]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(super) struct InstructionHeader {
        #[bits = 3]
        pub register: B3,
        pub immediate: bool,
        #[bits = 4]
        pub instruction: Instruction,
    }

    impl InstructionHeader {
        pub fn bits(&self) -> u8 {
            self.bytes[0]
        }
    }
}

#[derive(Debug, Clone)]
struct Control {
    head: InstructionHeader,
    clock: u8,
}

impl Default for Control {
    fn default() -> Self {
        // CPU starts up with 0x00 in all control registers
        Control {
            head: InstructionHeader::new(),
            clock: 0,
        }
    }
}

#[derive(Debug)]
struct State {
    pc: u16,
    sp: u16,
    ctrl: Control,
    sreg: SReg,
    timer: u16,
    alu: Alu,
    bus: u8,
    bank: RegBank,
    addr: u16,
    speed: Option<(Duration, u32)>,
    quit: bool,
    mem: Box<[u8]>,
    program: Box<[u8]>,
    peripherals: HashMap<u8, Peripheral>,
    text_buffer: TextBuffer,
}

impl State {
    fn init(program: Box<[u8]>) -> Self {
        State {
            pc: 0,
            sp: 0xEFFF,
            ctrl: Control::default(),
            sreg: SReg::empty(),
            timer: 0,
            alu: Alu::default(),
            bus: 0,
            bank: RegBank::default(),
            addr: 0,
            speed: None,
            quit: false,
            mem: vec![0; 1 << 16].into_boxed_slice(),
            program,
            peripherals: HashMap::new(),
            text_buffer: TextBuffer::spawn(),
        }
    }

    fn tick(&mut self) -> bool {
        if self.sreg.contains(SReg::H) {
            return true;
        }

        let cw = self.cw();

        if cw.contains(ControlWord::SH) {
            self.sreg.insert(SReg::H);
            return true;
        }

        // rising edge

        if cw.contains(ControlWord::PCI) {
            self.pc = self.pc.wrapping_add(1);
        }

        let program_byte = self.program[self.pc as usize];

        if cw.contains(ControlWord::LI) {
            let byte = program_byte;
            self.ctrl.head = InstructionHeader::from_bytes([byte]);
        }

        let reg_index = if cw.contains(ControlWord::RSB) {
            self.ctrl.head.register()
        } else if cw.contains(ControlWord::RSP) {
            program_byte & 0b0000_0111
        } else {
            0x00
        };

        let bus = if cw.contains(ControlWord::RBO) {
            self.bank.get_reg(reg_index)
        } else if cw.contains(ControlWord::AO) {
            self.alu.compute(
                cw.contains(ControlWord::AOL),
                cw.contains(ControlWord::AOM),
                cw.contains(ControlWord::AOH),
                &mut self.sreg,
            )
        } else if cw.contains(ControlWord::LA) {
            match self.addr {
                0x0000..=0xEFFF => self.mem[self.addr as usize],
                0xF000..=0xFFCF => self.text_buffer.get(self.addr - 0xF000),
                0xFFD0..=0xFFFC => match self.peripherals.get(&((self.addr - 0xFFC0) as u8)) {
                    Some(periph) => unsafe {
                        if let Ok(stateful_read) =
                            periph.lib.library.get::<StatefulReadFn>(b"stateful_read")
                        {
                            match periph.lib.state {
                                Some(state) => stateful_read(state, periph.n),
                                None => {
                                    eprintln!("PERIPHERAL ERROR: unable to call `stateful_read` (state was not initialized)");
                                    return true;
                                }
                            }
                        } else if let Ok(read) = periph.lib.library.get::<ReadFn>(b"read") {
                            read(periph.n)
                        } else {
                            eprintln!("PERIPHERAL ERROR: `read` and `stateful_write` not present in peripheral (peripherals must implement one of these)");
                            return true;
                        }
                    },
                    None => 0x00,
                },
                0xFFFD => (self.pc >> 8) as u8,
                0xFFFE => (self.pc & 0xFF) as u8,
                0xFFFF => self.sreg.bits(),
            }
        } else if cw.contains(ControlWord::PO) {
            program_byte
        } else if cw.contains(ControlWord::LPM) {
            self.program[self.addr as usize]
        } else {
            0x00
        };
        // Just for the 'DUMP' command.
        self.bus = bus;

        if cw.contains(ControlWord::LSP) {
            self.addr = self.sp;
        }

        if cw.contains(ControlWord::SA) {
            match self.addr {
                0x0000..=0xEFFF => {
                    self.mem[self.addr as usize] = bus;
                }
                0xF000..=0xFFCF => {
                    self.text_buffer.set(self.addr - 0xF000, bus);
                }
                0xFFD0..=0xFFFC => match self.peripherals.get(&((self.addr - 0xFFC0) as u8)) {
                    Some(periph) => unsafe {
                        if let Ok(stateful_write) =
                            periph.lib.library.get::<StatefulWriteFn>(b"stateful_write")
                        {
                            match periph.lib.state {
                                Some(state) => stateful_write(state, periph.n, bus),
                                None => {
                                    eprintln!("PERIPHERAL ERROR: unable to call `stateful_write` (state was not initialized)");
                                    return true;
                                }
                            }
                        } else if let Ok(write) = periph.lib.library.get::<WriteFn>(b"write") {
                            write(periph.n, bus);
                        } else {
                            eprintln!("PERIPHERAL ERROR: `read` and `stateful_read` not present in peripheral (peripherals must implement one of these)");
                            return true;
                        }
                    },
                    None => {}
                },
                0xFFFD => {
                    self.pc = (self.pc & 0xFF00) | (bus as u16);
                }
                0xFFFE => {
                    self.pc = (self.pc & 0x00FF) | ((bus as u16) << 8);
                }
                0xFFFF => {
                    self.sreg = SReg::from_bits_retain(bus);
                }
            }
        }

        if cw.contains(ControlWord::ALI) {
            self.addr = self.addr & !0xFF | bus as u16;
        }
        if cw.contains(ControlWord::AHI) {
            self.addr = self.addr & !0xFF00 | ((bus as u16) << 8);
        }

        if cw.contains(ControlWord::SR) {
            self.bank.set_reg(
                self.ctrl.head.register(),
                self.bank.get_reg(program_byte & 0b0000_0111),
            )
        }

        if cw.contains(ControlWord::THL | ControlWord::RBO) {
            self.addr = ((self.bank.h as u16) << 8) | (self.bank.l as u16);
        } else if cw.contains(ControlWord::THL | ControlWord::RBI) {
            let addr = self.addr.to_be_bytes();
            self.bank.h = addr[0];
            self.bank.l = addr[1];
        } else if cw.contains(ControlWord::RBI) {
            self.bank.set_reg(reg_index, bus);
        }

        if cw.contains(ControlWord::SPI) {
            self.sp = self.sp.wrapping_add(1);
        }

        if cw.contains(ControlWord::SPD) {
            self.sp = self.sp.wrapping_sub(1);
        }

        if !cw.contains(ControlWord::AO) {
            self.alu.execute(
                &mut self.sreg,
                bus,
                cw.contains(ControlWord::AOL),
                cw.contains(ControlWord::AOM),
                cw.contains(ControlWord::AOH),
            )
        }

        if cw.contains(ControlWord::JNZ) {
            if !self.sreg.contains(SReg::Z) {
                self.pc = ((self.bank.h as u16) << 8) | (self.bank.l as u16);
            } else if !cw.contains(ControlWord::PCI) {
                self.pc = self.pc.wrapping_add(1);
            }
        }

        // falling edge
        for (_, peripheral) in self.peripherals.iter() {
            unsafe {
                if let Ok(stateful_tick) = peripheral
                    .lib
                    .library
                    .get::<StatefulTickFn>(b"stateful_tick")
                {
                    match peripheral.lib.state {
                        Some(state) => stateful_tick(state),
                        None => {
                            eprintln!("PERIPHERAL ERROR: unable to call `stateful_tick` (state was not initialized)");
                        }
                    }
                } else if let Ok(stateless_tick) = peripheral.lib.library.get::<TickFn>(b"tick") {
                    stateless_tick();
                }
            }
        }

        self.timer = self.timer.wrapping_add(1);

        if cw.contains(ControlWord::CR) {
            self.ctrl.clock = 0;
        } else {
            self.ctrl.clock = self.ctrl.clock.wrapping_add(1);
        }

        false
    }

    fn halt(&mut self) {
        self.speed = None;

        print!(
            "INFO: halt detected\n\
            > "
        );
        std::io::stdout()
            .flush()
            .expect("should be able to write to `stdout`");
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.ctrl.clock = 0;
        self.mem.fill(0);
        self.sreg = SReg::from_bits_retain(0);
        self.sp = 0xEFFF;
        self.alu.clear();
        self.bank.clear();
        self.text_buffer.reset();

        for periph in self.peripherals.values() {
            unsafe {
                if let Ok(stateful_reset) =
                    periph.lib.library.get::<StatefulResetFn>(b"stateful_reset")
                {
                    match periph.lib.state {
                        Some(state) => stateful_reset(state),
                        None => {
                            eprintln!("PERIPHERAL ERROR: unable to call `stateful_reset` (state was not initialized)");
                        }
                    }
                } else if let Ok(reset) = periph.lib.library.get::<ResetFn>(b"reset") {
                    reset();
                }
            }
        }
    }

    fn cw(&self) -> ControlWord {
        let index = ((self.ctrl.head.instruction() as u8) << 4
            | (self.ctrl.head.immediate() as u8) << 3
            | self.ctrl.clock) as usize;

        let low = CTRL_LOW[index] as u32;
        let mid = CTRL_MID[index] as u32;
        let high = CTRL_HIGH[index] as u32;

        ControlWord::from_bits_retain(low | (mid << 8) | (high << 16))
    }

    fn load(&mut self, path: String, ports: Vec<u8>) {
        let (library, name, state) = unsafe {
            let lib = match Library::new(&path) {
                Ok(lib) => lib,
                Err(err) => {
                    eprintln!("Unable to load peripheral library: {err:?}");
                    return;
                }
            };

            let name = match lib.get::<NameFn>(b"name") {
                Ok(name) => CStr::from_ptr(name()).to_str().ok().map(|s| s.to_owned()),
                Err(_) => None,
            }
            .unwrap_or(path);

            let (state, err) =
                if let Ok(stateful_init) = lib.get::<StatefulInitFn>(b"stateful_init") {
                    let state = stateful_init(ports.len() as u8);
                    (Some(state), if state.is_null() { -1 } else { 0 })
                } else if let Ok(init) = lib.get::<InitFn>(b"init") {
                    let err = init(ports.len() as u8);
                    (None, err)
                } else {
                    (None, 0)
                };

            if err != 0 {
                let msg: Result<(Vec<u8>, c_int), libloading::Error> = lib
                    .get::<LastErrLen>(b"last_error_length")
                    .map(|lel| vec![0; lel() as usize])
                    .and_then(|mut buf| {
                        let written = lib.get::<GetLastErr>(b"get_last_error")?(
                            buf.as_mut_ptr() as *mut c_char,
                            buf.len() as c_int,
                        );
                        Ok((buf, written))
                    });

                match (msg, state.is_some()) {
                    (Ok((buffer, 1..)), st) => match CStr::from_bytes_with_nul_unchecked(&buffer).to_str() {
                        Ok(msg) => eprintln!("PERIPHERAL ERROR: {msg}"),
                        Err(_) => match st {
                            true => eprintln!("PERIPHRAL ERROR: initialization failed (`stateful_init` returned a null pointer)"),
                            false => eprintln!("PERIPHERAL ERROR: initialization failed (`init` returned with exit code {err})"),
                        }
                    },
                    (_, true) => eprintln!("PERIPHRAL ERROR: initialization failed (`stateful_init` returned a null pointer)"),
                    (_, false) => eprintln!("PERIPHERAL ERROR: initialization failed (`init` returned with exit code {err})"),
                }
                return;
            }

            (lib, name, state)
        };

        let lib = Arc::new(Lib {
            library,
            name,
            state,
        });

        for (n, port) in ports.into_iter().enumerate() {
            let periph = Peripheral {
                lib: lib.clone(),
                n: n as u8,
            };
            if let Some(periph) = self.peripherals.insert(port, periph) {
                println!(
                    "WARNING: overwriting previous peripheral `{}`",
                    periph.lib.name
                );
            }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let periph = if self.peripherals.is_empty() {
            "[]".to_owned()
        } else {
            "[\n    ".to_owned()
                + &self
                    .peripherals
                    .iter()
                    .map(|(key, periph)| {
                        format!("{:#06X}: \"{}\"", (*key as u16) + 0xFFC0, periph.lib.name)
                    })
                    .collect::<Vec<String>>()
                    .join(",\n    ")
                + ",\n]"
        };

        write!(
            f,
            "\
                PROGRAM COUNTER: {:#06X}\n\
                STACK POINTER: {:#06X}\n\
                BUS: {:#04X}\n\
                SREG: {:#04X}\n\
                PROGRAM BYTE: {:#04X}\n\
                ALU PRIMARY: {:#04X}\n\
                ALU SECONDARY: {:#04X}\n\
                {}\
                CONTROL WORD: {:?}\n\
                INSTRUCTION: {:#010b}\n\
                PERIPHERALS: {periph}\n\
            ",
            self.pc,
            self.sp,
            self.bus,
            self.sreg.bits(),
            self.program[self.pc as usize],
            self.alu.primary,
            self.alu.secondary,
            self.bank,
            self.cw(),
            self.ctrl.head.bits()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum ZeroCmd {
    Dump,
    Quit,
    Step,
    Reset,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum SingleCmd {
    Get,
    Peek,
    Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum DoubleCmd {
    Set,
    Poke,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum VariadicCmd {
    Load,
    Drop,
    Help,
}

#[derive(Debug)]
struct Lib {
    name: String,
    library: Library,
    state: Option<*mut c_void>,
}

impl Drop for Lib {
    fn drop(&mut self) {
        unsafe {
            if let Ok(stateful_drop) = self.library.get::<StatefulDropFn>(b"stateful_drop") {
                match self.state {
                    Some(state) => stateful_drop(state),
                    None => {
                        eprintln!("PERIPHERAL ERROR: unable to call `stateful_drop` (state was not initialized)");
                    }
                }
            } else if let Ok(stateless_drop) = self.library.get::<DropFn>(b"drop") {
                stateless_drop();
            }
        }
    }
}

// We can do this since we are not using `state` across threads..
unsafe impl Send for Lib {}
unsafe impl Sync for Lib {}

#[derive(Debug)]
struct Peripheral {
    lib: Arc<Lib>,
    n: u8,
}

static STATE: OnceLock<RwLock<State>> = OnceLock::new();

pub async fn emulate(mut args: EmulatorArgs) -> Result<(), EmulatorError> {
    let mut program: Box<[u8]> = vec![0; 1 << 16].into_boxed_slice();

    args.input
        .read(&mut program)
        .map_err(|err| EmulatorError::Input(err))?;

    STATE
        .set(RwLock::new(State::init(program)))
        .map_err(|_| EmulatorError::OnceFull)?;

    print!("> ");
    std::io::stdout()
        .flush()
        .map_err(|err| EmulatorError::StdOut(err))?;
    let stdin = spawn_stdin_channel();

    let mut prev = Instant::now();

    loop {
        match stdin.try_recv() {
            Ok(s) => {
                handle_input(s, &mut std::io::stdout(), &mut std::io::stderr()).await?;
                std::io::stdout()
                    .flush()
                    .map_err(|err| EmulatorError::StdOut(err))?;
            }
            Err(TryRecvError::Closed) => return Err(EmulatorError::StdIn),
            Err(TryRecvError::Empty) => {}
        }

        if STATE
            .get()
            .ok_or(EmulatorError::OnceEmpty)?
            .read()
            .await
            .quit
        {
            break;
        }

        let mut state = STATE.get().ok_or(EmulatorError::OnceEmpty)?.write().await;

        let tick = match state.speed {
            Some(speed) => prev.elapsed() >= speed.0,
            None => false,
        };

        if tick {
            prev = Instant::now();

            let halted = state.tick();
            if halted {
                state.halt();
            }
        } else {
            async_std::task::sleep(Duration::from_millis(10)).await;
        }
    }

    Ok(())
}

async fn handle_input(
    input: String,
    mut writer: impl std::io::Write,
    mut ewriter: impl std::io::Write,
) -> Result<(), EmulatorError> {
    let (cmd, arg_string) = input.trim().split_once(' ').unwrap_or((&input.trim(), ""));
    let args: Vec<&str> = arg_string
        .split(' ')
        .filter(|arg| arg.trim().len() > 0)
        .collect();
    match cmd {
        "GET" => single_arg(SingleCmd::Get, &args, &mut writer, ewriter).await?,
        "SET" => double_arg(DoubleCmd::Set, &args, &mut writer, ewriter).await?,
        "PEEK" => single_arg(SingleCmd::Peek, &args, &mut writer, ewriter).await?,
        "POKE" => double_arg(DoubleCmd::Poke, &args, &mut writer, ewriter).await?,
        "DROP" => variadic_arg(VariadicCmd::Drop, &args, &mut writer, ewriter).await?,
        "RUN" => single_arg(SingleCmd::Run, &args, &mut writer, ewriter).await?,
        "LOAD" => variadic_arg(VariadicCmd::Load, &args, &mut writer, ewriter).await?,
        "DUMP" => zero_arg(ZeroCmd::Dump, &args, &mut writer, ewriter).await?,
        "QUIT" => zero_arg(ZeroCmd::Quit, &args, &mut writer, ewriter).await?,
        "STEP" => zero_arg(ZeroCmd::Step, &args, &mut writer, ewriter).await?,
        "RESET" => zero_arg(ZeroCmd::Reset, &args, &mut writer, ewriter).await?,
        "STOP" => zero_arg(ZeroCmd::Stop, &args, &mut writer, ewriter).await?,
        "HELP" => variadic_arg(VariadicCmd::Help, &args, &mut writer, &mut ewriter).await?,
        "" => {}
        cmd => writeln!(ewriter, "UNRECOGNIZED COMMAND: {cmd}")
            .map_err(|err| EmulatorError::StdOut(err))?,
    }

    write!(writer, "> ").map_err(|err| EmulatorError::StdOut(err))?;

    Ok(())
}

pub fn test_emulate(program: Box<[u8]>, timeout: Duration) -> Result<RegBank, ()> {
    let start = Instant::now();
    let mut state = State::init(program);

    while start.elapsed() <= timeout {
        let halted = state.tick();
        if halted {
            return Ok(state.bank);
        }
    }

    Err(())
}

fn help() {
    println!(
        "\
        SET <reg>, <val>    : Sets the value in the register `reg` to `val`\n\
        GET <reg>           : Gets the value in the register `reg`\n\
        PEEK <addr>         : Gets the value at the memory address `addr`\n\
        POKE <addr>, <val>  : Sets the value at the memory address `addr` to `val`\n\
        RUN <speed>         : Starts running the CPU at the specified `speed` (in hertz)\n\
        LOAD <path>, <port> : Loads the library at the given path as a peripheral.\n\
        DROP <port>         : Disconnects the peripheral on the given port, unloading the module.\n\
        DUMP                : Dumps the current machine state\n\
        STEP                : Pulses the clock a single time (only available if the CPU is stopped)\n\
        RESET               : Resets the program counter to 0x0000\n\
        STOP                : Stops the CPU clock\n\
        QUIT                : Quits the program\n\
        HELP                : Prints this message\
    "
    );
}

async fn zero_arg(
    cmd: ZeroCmd,
    args: &[&str],
    mut writer: impl std::io::Write,
    mut ewriter: impl std::io::Write,
) -> Result<(), EmulatorError> {
    let count = args.into_iter().filter(|arg| arg.trim().len() > 0).count();
    if count != 0 {
        writeln!(
            ewriter,
            "ARGUMENT ERROR: expected no arguments, found {count}"
        )
        .map_err(|err| EmulatorError::StdOut(err))?;
        return Ok(());
    }

    match cmd {
        ZeroCmd::Dump => write!(
            writer,
            "{}",
            STATE.get().ok_or(EmulatorError::OnceEmpty)?.read().await
        )
        .map_err(|err| EmulatorError::StdOut(err))?,
        ZeroCmd::Quit => {
            STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .write()
                .await
                .quit = true;
            return Ok(());
        }
        ZeroCmd::Step => {
            if STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .read()
                .await
                .speed
                .is_none()
            {
                let halted = STATE
                    .get()
                    .ok_or(EmulatorError::OnceEmpty)?
                    .write()
                    .await
                    .tick();

                if halted {
                    writeln!(
                        ewriter,
                        "INVALID COMMAND: CPU is halted, the clock cannot be stepped"
                    )
                    .map_err(|err| EmulatorError::StdOut(err))?;
                }
            } else {
                writeln!(
                    ewriter,
                    "INVALID COMMAND: STEP can only be used if the CPU is stopped"
                )
                .map_err(|err| EmulatorError::StdOut(err))?;
            }
        }
        ZeroCmd::Reset => STATE
            .get()
            .ok_or(EmulatorError::OnceEmpty)?
            .write()
            .await
            .reset(),
        ZeroCmd::Stop => {
            STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .write()
                .await
                .speed = None
        }
    }

    Ok(())
}

async fn single_arg(
    cmd: SingleCmd,
    args: &[&str],
    mut writer: impl std::io::Write,
    mut ewriter: impl std::io::Write,
) -> Result<(), EmulatorError> {
    let count = args.into_iter().filter(|arg| arg.trim().len() > 0).count();
    if count != 1 {
        writeln!(
            ewriter,
            "ARGUMENT ERROR: expected 1 argument, found {count}"
        )
        .map_err(|err| EmulatorError::StdOut(err))?;
        return Ok(());
    }
    let arg = args[0].trim();

    match cmd {
        SingleCmd::Get => {
            let reg = match parse_u8(arg) {
                Ok(reg) => reg,
                Err(_) => match arg {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    "D" => 3,
                    "E" => 4,
                    "H" => 5,
                    "L" => 6,
                    "F" => 7,
                    _ => {
                        writeln!(ewriter, "INVALID ARGUMENT: unable to parse register")
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        return Ok(());
                    }
                },
            };
            if reg >= 8 {
                writeln!(ewriter, "INVALID ARGUMENT: register out of range")
                    .map_err(|err| EmulatorError::StdOut(err))?;
                return Ok(());
            }
            writeln!(
                writer,
                "REGISTER {reg}: {:#04X}",
                STATE
                    .get()
                    .ok_or(EmulatorError::OnceEmpty)?
                    .read()
                    .await
                    .bank
                    .get_reg(reg)
            )
            .map_err(|err| EmulatorError::StdOut(err))?
        }
        SingleCmd::Peek => {
            let addr = match parse_u16(arg) {
                Ok(addr) => addr,
                Err(_) => {
                    writeln!(ewriter, "INVALID ARGUMENT: unable to parse address")
                        .map_err(|err| EmulatorError::StdOut(err))?;
                    return Ok(());
                }
            };

            let data: u8 = match addr {
                0x0000..=0xFFBF => {
                    STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .read()
                        .await
                        .mem[addr as usize]
                }
                0xFFC0..=0xFFFC => {
                    match STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .read()
                        .await
                        .peripherals
                        .get(&((addr - 0xFFC0) as u8))
                    {
                        Some(periph) => unsafe {
                            if let Ok(stateful_read) =
                                periph.lib.library.get::<StatefulReadFn>(b"stateful_read")
                            {
                                match periph.lib.state {
                                    Some(state) => stateful_read(state, periph.n),
                                    None => {
                                        writeln!(ewriter, "PERIPHERAL ERROR: unable to call `stateful_read` (state was not initialized)").map_err(|err| EmulatorError::StdOut(err))?;
                                        return Ok(());
                                    }
                                }
                            } else if let Ok(read) = periph.lib.library.get::<ReadFn>(b"read") {
                                read(periph.n)
                            } else {
                                writeln!(ewriter, "PERIPHERAL ERROR: `read` and `stateful_read` not present in peripheral (peripherals must implement one of these)").map_err(|err| EmulatorError::StdOut(err))?;
                                return Ok(());
                            }
                        },
                        None => 0x00,
                    }
                }
                0xFFFD => {
                    (STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .read()
                        .await
                        .timer
                        >> 8) as u8
                }
                0xFFFE => {
                    (STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .read()
                        .await
                        .timer
                        & 0xFF) as u8
                }
                0xFFFF => STATE
                    .get()
                    .ok_or(EmulatorError::OnceEmpty)?
                    .read()
                    .await
                    .sreg
                    .bits(),
            };

            println!("{addr:#06X}: {data:#04X}");
        }
        SingleCmd::Run => {
            let speed = match parse_u32(arg) {
                Ok(speed) => speed,
                Err(_) => {
                    writeln!(ewriter, "INVALID ARGUMENT: unable to parse speed")
                        .map_err(|err| EmulatorError::StdOut(err))?;
                    return Ok(());
                }
            };

            let duration = if speed == 0 {
                Duration::ZERO
            } else {
                Duration::from_secs(1) / speed
            };

            STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .write()
                .await
                .speed = Some((duration, speed));
        }
    }

    Ok(())
}

async fn double_arg(
    cmd: DoubleCmd,
    args: &[&str],
    mut _writer: impl std::io::Write,
    mut ewriter: impl std::io::Write,
) -> Result<(), EmulatorError> {
    let count = args.into_iter().filter(|arg| arg.trim().len() > 0).count();
    if count != 2 {
        writeln!(
            ewriter,
            "ARGUMENT ERROR: expected `2` arguments, found `{count}`"
        )
        .map_err(|err| EmulatorError::StdOut(err))?;
        return Ok(());
    }
    let arg1 = args[0].trim();
    let arg2 = args[1].trim();

    match cmd {
        DoubleCmd::Set => {
            let reg = match parse_u8(arg1.trim()) {
                Ok(reg) => reg,
                Err(_) => match arg1.trim() {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    "D" => 3,
                    "E" => 4,
                    "H" => 5,
                    "L" => 6,
                    "F" => 7,
                    _ => {
                        writeln!(ewriter, "INVALID ARGUMENT: unable to parse register")
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        return Ok(());
                    }
                },
            };
            if reg >= 8 {
                writeln!(ewriter, "INVALID ARGUMENT: register out of range")
                    .map_err(|err| EmulatorError::StdOut(err))?;
                return Ok(());
            }

            let value = match parse_u8(arg2.trim()) {
                Ok(val) => val,
                Err(_) => {
                    writeln!(ewriter, "INVALID ARGUMENT: unable to parse value")
                        .map_err(|err| EmulatorError::StdOut(err))?;
                    return Ok(());
                }
            };

            STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .write()
                .await
                .bank
                .set_reg(reg, value);
        }
        DoubleCmd::Poke => {
            let addr = match parse_u16(arg1.trim()) {
                Ok(addr) => addr,
                Err(_) => {
                    writeln!(ewriter, "INVALID ARGUMENT: unable to parse address")
                        .map_err(|err| EmulatorError::StdOut(err))?;
                    return Ok(());
                }
            };

            let value = match parse_u8(arg2.trim()) {
                Ok(val) => val,
                Err(_) => {
                    writeln!(ewriter, "INVALID ARGUMENT: unable to parse value")
                        .map_err(|err| EmulatorError::StdOut(err))?;
                    return Ok(());
                }
            };

            match addr {
                0x0000..=0xFFBF => {
                    STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .write()
                        .await
                        .mem[addr as usize] = value
                }
                0xFFC0..=0xFFFC => {
                    match STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .read()
                        .await
                        .peripherals
                        .get(&((addr - 0xFFC0) as u8))
                    {
                        Some(periph) => unsafe {
                            if let Ok(stateful_write) =
                                periph.lib.library.get::<StatefulWriteFn>(b"stateful_write")
                            {
                                match periph.lib.state {
                                    Some(state) => stateful_write(state, periph.n, value),
                                    None => {
                                        writeln!(ewriter, "PERIPHERAL ERROR: unable to call `stateful_read` (state was not initialized)").map_err(|err| EmulatorError::StdOut(err))?;
                                        return Ok(());
                                    }
                                };
                            } else if let Ok(write) = periph.lib.library.get::<WriteFn>(b"write") {
                                write(periph.n, value);
                            } else {
                                writeln!(ewriter, "PERIPHERAL ERROR: `write` and `stateful_write` not present in peripheral (peripherals must implement one of these)").map_err(|err| EmulatorError::StdOut(err))?;
                                return Ok(());
                            }
                        },
                        None => {}
                    }
                }
                0xFFFD => {
                    let mut state = STATE.get().ok_or(EmulatorError::OnceEmpty)?.write().await;
                    state.timer = (state.timer & 0x00FF) | ((value as u16) << 8)
                }
                0xFFFE => {
                    let mut state = STATE.get().ok_or(EmulatorError::OnceEmpty)?.write().await;
                    state.timer = (state.timer & 0xFF00) | ((value & 0xFF) as u16)
                }
                0xFFFF => {
                    STATE
                        .get()
                        .ok_or(EmulatorError::OnceEmpty)?
                        .write()
                        .await
                        .sreg = SReg::from_bits_retain(value)
                }
            };
        }
    }

    Ok(())
}

async fn variadic_arg(
    cmd: VariadicCmd,
    args: &[&str],
    mut writer: impl std::io::Write,
    mut ewriter: impl std::io::Write,
) -> Result<(), EmulatorError> {
    match cmd {
        VariadicCmd::Load => {
            if args.is_empty() {
                writeln!(
                    ewriter,
                    "ARGUMENT ERROR: expected at least `1` argument, found `0`"
                )
                .map_err(|err| EmulatorError::StdOut(err))?;
                return Ok(());
            } else if args.len() == 1 {
                writeln!(
                    writer,
                    "WARNING: no ports provided. {} will not be initialized",
                    args[0]
                )
                .map_err(|err| EmulatorError::StdOut(err))?;
                return Ok(());
            }
            let (path_slice, args_slice) = args.split_at(1);

            let path = parse_path(path_slice[0]);

            let mut ports = Vec::new();
            for addr in args_slice.into_iter().filter(|arg| arg.trim().len() > 0) {
                match parse_u16(addr) {
                    Ok(p) => {
                        if p < 0xFFC0 {
                            writeln!(ewriter, "INVALID ARGUMENT: memory mapped I/O can only be mapped to addresses between 0xFFC0-0xFFFF").map_err(|err| EmulatorError::StdOut(err))?;
                            return Ok(());
                        } else if p == 0xFFFF {
                            writeln!(
                                ewriter,
                                "INVALID ARGUMENT: the status register cannot be overwritten"
                            )
                            .map_err(|err| EmulatorError::StdOut(err))?;
                            return Ok(());
                        } else if p == 0xFFFE || p == 0xFFFD {
                            writeln!(
                                ewriter,
                                "INVALID ARGUMENT: the stack pointer cannot be overwritten"
                            )
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        } else {
                            ports.push((p - 0xFFC0) as u8)
                        }
                    }
                    Err(_) => {
                        writeln!(ewriter, "INVALID ARGUMENT: unable to parse address {addr}")
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        return Ok(());
                    }
                }
            }

            STATE
                .get()
                .ok_or(EmulatorError::OnceEmpty)?
                .write()
                .await
                .load(path, ports);
        }
        VariadicCmd::Drop => {
            if args.is_empty() {
                writeln!(writer, "INFO: dropping all peripherals")
                    .map_err(|err| EmulatorError::StdOut(err))?;
                STATE
                    .get()
                    .ok_or(EmulatorError::OnceEmpty)?
                    .write()
                    .await
                    .peripherals
                    .clear();
                return Ok(());
            }

            let mut ports = Vec::new();
            for addr in args.into_iter().filter(|arg| arg.trim().len() > 0) {
                match parse_u16(addr) {
                    Ok(p) => {
                        if p < 0xFFC0 {
                            writeln!(ewriter, "INVALID ARGUMENT: memory mapped I/O can only be mapped to addresses between 0xFFC0-0xFFFF").map_err(|err| EmulatorError::StdOut(err))?;
                            return Ok(());
                        } else if p == 0xFFFF {
                            writeln!(
                                ewriter,
                                "INVALID ARGUMENT: the status register cannot be overwritten"
                            )
                            .map_err(|err| EmulatorError::StdOut(err))?;
                            return Ok(());
                        } else if p == 0xFFFE || p == 0xFFFD {
                            writeln!(
                                ewriter,
                                "INVALID ARGUMENT: the stack pointer cannot be overwritten"
                            )
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        } else {
                            ports.push((p - 0xFFC0) as u8)
                        }
                    }
                    Err(_) => {
                        writeln!(ewriter, "INVALID ARGUMENT: unable to parse address {addr}")
                            .map_err(|err| EmulatorError::StdOut(err))?;
                        return Ok(());
                    }
                }
            }

            let mut state = STATE.get().ok_or(EmulatorError::OnceEmpty)?.write().await;

            for port in ports {
                if let None = state.peripherals.remove(&port) {
                    writeln!(
                        writer,
                        "WARNING: no peripheral found at address {:#06X}",
                        (port as u16) + 0xFFBF
                    )
                    .map_err(|err| EmulatorError::StdOut(err))?;
                }
            }
        }
        VariadicCmd::Help => {
            if args.is_empty() {
                help();
            } else {
                for arg in args {
                    let command = match Command::from_str(arg) {
                        Ok(c) => c,
                        Err(_) => {
                            writeln!(ewriter, "ARGUMENT ERROR: unrecognized command `{arg}`")
                                .map_err(|err| EmulatorError::StdOut(err))?;
                            return Ok(());
                        }
                    };

                    write!(writer, "\n{}\n", command.help())
                        .map_err(|err| EmulatorError::StdOut(err))?;
                }
            }
        }
    }

    Ok(())
}

fn parse_u8(int: &str) -> Result<u8, <u8 as FromStr>::Err> {
    if int.starts_with("0b") {
        u8::from_str_radix(&int[2..], 2)
    } else if int.starts_with("0o") {
        u8::from_str_radix(&int[2..], 8)
    } else if int.starts_with("0x") {
        u8::from_str_radix(&int[2..], 16)
    } else {
        u8::from_str_radix(int, 10)
    }
}

fn parse_u16(int: &str) -> Result<u16, <u16 as FromStr>::Err> {
    if int.starts_with("0b") {
        u16::from_str_radix(&int[2..], 2)
    } else if int.starts_with("0o") {
        u16::from_str_radix(&int[2..], 8)
    } else if int.starts_with("0x") {
        u16::from_str_radix(&int[2..], 16)
    } else {
        u16::from_str_radix(int, 10)
    }
}

fn parse_u32(int: &str) -> Result<u32, <u32 as FromStr>::Err> {
    if int.starts_with("0b") {
        u32::from_str_radix(&int[2..], 2)
    } else if int.starts_with("0o") {
        u32::from_str_radix(&int[2..], 8)
    } else if int.starts_with("0x") {
        u32::from_str_radix(&int[2..], 16)
    } else {
        u32::from_str_radix(int, 10)
    }
}

fn parse_path(source: &str) -> String {
    source.trim_matches('"').trim_matches('\'').to_owned()
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = channel::unbounded();
    async_std::task::spawn(watch_input(tx));
    rx
}

async fn watch_input(tx: Sender<String>) {
    loop {
        let mut buffer = String::new();
        // we unwrap here, since when this thread panics,
        // the error will bubble up through the message channel
        io::stdin().read_line(&mut buffer).await.unwrap();
        tx.send(buffer).await.unwrap();
    }
}
