// Didn't make actual good error reporting since I'm the only one who'll be debugging this.

use bitflags::bitflags;
use logos::{Lexer, Logos};
use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;
use std::{env, fs};
use thiserror::Error;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r";[^\n]*")]
#[logos(skip r"[ \r\t\f]+")]
enum Token {
    #[regex(r"[._a-zA-Z][_a-zA-Z0-9]*:?", |lex| lex.slice().to_owned())]
    Ident(String),
    #[token("|")]
    Pipe,
    #[token("\n")]
    Newline,
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Shadow(#[from] shadow_rs::ShadowError),
    #[error("unable to locate output directory")]
    OutDir,
    #[error(transparent)]
    Fs(#[from] std::io::Error),
    #[error("Unknown instruction: {0}")]
    Instruction(String),
    #[error("Unknown section: {0}")]
    Section(String),
    #[error("Unknown flag: {0}")]
    UnknownFlag(String),
    #[error("Expected `|`, found {0}")]
    UnexpectedFlag(String),
    #[error("Expected newline after")]
    Newline,
    #[error("Expected flag, found `|`")]
    Pipe,
    #[error("Top level cycles not allowed")]
    Top,
    #[error("Unknown token encountered at index: {0:?}")]
    Lex(Range<usize>),
}

fn main() -> Result<(), Error> {
    shadow_rs::new()?;

    println!("cargo:rerun-if-changed=src/microcode.asm");
    println!("cargo:rerun-if-changed=build.rs");

    create_display_multiplier()?;

    let file = fs::read_to_string("src/microcode.asm")?;
    let lex = Token::lexer(&file);
    let stream = Stream::parse(lex)?;
    let microcode = stream.stitch();

    println!("{:?}", microcode[0b0110_1000]);
    println!("{:?}", microcode[0b0110_1001]);
    println!("{:?}", microcode[0b0110_1010]);
    println!("{:?}", microcode[0b0110_1011]);

    let (ctrl_low, (ctrl_mid, ctrl_high)): (Vec<u8>, (Vec<u8>, Vec<u8>)) = microcode
        .into_iter()
        .map(|cw| {
            (
                (cw.bits() & 0xFF) as u8,
                (
                    ((cw.bits() >> 8) & 0xFF) as u8,
                    ((cw.bits() >> 16) & 0xFF) as u8,
                ),
            )
        })
        .unzip();

    let out_env = env::var_os("OUT_DIR").ok_or(Error::OutDir)?;
    let out_dir = Path::new(&out_env);
    fs::write(out_dir.join("ctrl_low.rom"), &ctrl_low)?;
    fs::write(out_dir.join("ctrl_mid.rom"), &ctrl_mid)?;
    fs::write(out_dir.join("ctrl_high.rom"), &ctrl_high)?;

    Ok(())
}

fn create_display_multiplier() -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
struct Sequence {
    start: Vec<ControlWord>,
    reg: Vec<ControlWord>,
    imm: Vec<ControlWord>,
    end: Vec<ControlWord>,
}

impl Sequence {
    fn empty() -> Self {
        Sequence {
            start: Vec::new(),
            reg: Vec::new(),
            imm: Vec::new(),
            end: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Stream {
    instructions: HashMap<Instruction, Sequence>,
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

impl FromStr for ControlWord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "aol" => Ok(ControlWord::AOL),
            "aom" => Ok(ControlWord::AOM),
            "aoh" => Ok(ControlWord::AOH),
            "ao" => Ok(ControlWord::AO),
            "rbi" => Ok(ControlWord::RBI),
            "rbo" => Ok(ControlWord::RBO),
            "rsb" => Ok(ControlWord::RSB),
            "rsp" => Ok(ControlWord::RSP),
            "spi" => Ok(ControlWord::SPI),
            "spd" => Ok(ControlWord::SPD),
            "cr" => Ok(ControlWord::CR),
            "pci" => Ok(ControlWord::PCI),
            "jnz" => Ok(ControlWord::JNZ),
            "li" => Ok(ControlWord::LI),
            "po" => Ok(ControlWord::PO),
            "sr" => Ok(ControlWord::SR),
            "thl" => Ok(ControlWord::THL),
            "la" => Ok(ControlWord::LA),
            "sa" => Ok(ControlWord::SA),
            "ali" => Ok(ControlWord::ALI),
            "ahi" => Ok(ControlWord::AHI),
            "lsp" => Ok(ControlWord::LSP),
            "lpm" => Ok(ControlWord::LPM),
            "sh" => Ok(ControlWord::SH),
            _ => Err(Error::UnknownFlag(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Start,
    Reg,
    Imm,
    End,
}

impl Stream {
    fn parse(lex: Lexer<Token>) -> Result<Self, Error> {
        let mut instructions = HashMap::new();
        let mut current_instr = None;
        let mut newline = false;
        let mut pipe = false;
        let mut current_sequence = Sequence::empty();
        let mut current_section = Section::Start;
        let mut current_cw = None;

        for (token, span) in lex.spanned() {
            match token {
                Ok(tok) => match tok {
                    Token::Ident(i) => {
                        if i.starts_with('.') && i.ends_with(':') {
                            if current_instr.is_some() && newline {
                                current_section = match i.as_str() {
                                    ".start:" => Section::Start,
                                    ".reg:" => Section::Reg,
                                    ".imm:" => Section::Imm,
                                    ".both:" | ".end:" => Section::End,
                                    _ => return Err(Error::Section(i)),
                                };
                            } else {
                                if current_instr.is_some() {
                                    return Err(Error::Newline);
                                } else {
                                    return Err(Error::Top);
                                }
                            }
                        } else if i.ends_with(':') {
                            if let Some(instr) = current_instr {
                                instructions.insert(instr, current_sequence);
                                current_sequence = Sequence::empty();
                            }

                            current_instr = Some(match i.as_str() {
                                "add:" => Instruction::Add,
                                "sub:" => Instruction::Sub,
                                "adc:" => Instruction::Adc,
                                "sbb:" => Instruction::Sbb,
                                "nand:" => Instruction::Nand,
                                "or:" => Instruction::Or,
                                "cmp:" => Instruction::Cmp,
                                "mv:" => Instruction::Mv,
                                "ld:" => Instruction::Ld,
                                "st:" => Instruction::St,
                                "lda:" => Instruction::Lda,
                                "lpm:" => Instruction::Lpm,
                                "push:" => Instruction::Push,
                                "pop:" => Instruction::Pop,
                                "jnz:" => Instruction::Jnz,
                                "halt:" => Instruction::Halt,
                                _ => return Err(Error::Instruction(i)),
                            });
                            newline = false;
                            current_section = Section::Start;
                        } else {
                            match current_cw {
                                Some(ref mut cw) => {
                                    if pipe {
                                        *cw |= ControlWord::from_str(&i)?;
                                        pipe = false;
                                    } else {
                                        return Err(Error::UnexpectedFlag(i));
                                    }
                                }
                                None => current_cw = Some(ControlWord::from_str(&i)?),
                            }
                        }
                    }
                    Token::Newline => {
                        if current_instr.is_some() {
                            if newline && !pipe {
                                if let Some(cw) = current_cw {
                                    match current_section {
                                        Section::Start => current_sequence.start.push(cw),
                                        Section::Reg => current_sequence.reg.push(cw),
                                        Section::Imm => current_sequence.imm.push(cw),
                                        Section::End => current_sequence.end.push(cw),
                                    }
                                    current_cw = None;
                                }
                            } else {
                                newline = true;
                            }
                        }
                    }
                    Token::Pipe => {
                        if current_cw.is_some() {
                            pipe = true
                        } else {
                            return Err(Error::Pipe);
                        }
                    }
                },
                Err(_) => return Err(Error::Lex(span)),
            }
        }

        if let Some(instr) = current_instr {
            if let Some(cw) = current_cw {
                match current_section {
                    Section::Start => current_sequence.start.push(cw),
                    Section::Reg => current_sequence.reg.push(cw),
                    Section::Imm => current_sequence.imm.push(cw),
                    Section::End => current_sequence.end.push(cw),
                }
            }

            instructions.insert(instr, current_sequence);
        }

        Ok(Stream { instructions })
    }

    fn stitch(self) -> [ControlWord; 1 << 8] {
        let mut ctrl = [ControlWord::empty(); 1 << 8];

        println!("{:?}", self.instructions.get(&Instruction::Mv));

        for (instr, seq) in self.instructions {
            let base = (instr as u8) << 4;
            let mut reg = base;
            let mut imm = base | 0b1000;

            if instr == Instruction::Mv {
                println!("{base}");
            }

            for cw in seq.start {
                ctrl[reg as usize] = cw;
                reg += 1;

                ctrl[imm as usize] = cw;
                imm += 1;
            }

            for cw in seq.reg {
                ctrl[reg as usize] = cw;
                reg += 1;
            }

            for cw in seq.imm {
                ctrl[imm as usize] = cw;
                imm += 1;
            }

            for cw in seq.end {
                ctrl[reg as usize] = cw;
                reg += 1;

                ctrl[imm as usize] = cw;
                imm += 1;
            }
        }

        ctrl
    }
}
