mod emulator;
use std::{
    process::{ExitCode, Termination},
    sync::OnceLock,
};

use emulator::{EmulatorArgs, EmulatorError};
mod deploy;
use deploy::{DeployArgs, DeployError};
mod assembler;
use assembler::{AssemblerArgs, AssemblerError};
mod tests;
use tests::TestArgs;

mod diagnostic;
use diagnostic::ResultScream;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Level, WarnLevel};
use shadow_rs::shadow;

shadow!(build);

/// Enables program creation for the F8ful CPU.
#[derive(Parser, Debug)]
#[command(name = "Fateful", author, version = build::CLAP_LONG_VERSION, about)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<WarnLevel>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the F8ful emulator
    #[clap(alias = "emu")]
    Emulate(EmulatorArgs),
    /// Deploy a program to a connected CPU
    Deploy(DeployArgs),
    /// Assemble a Fate program
    #[clap(alias = "asm")]
    Assemble(AssemblerArgs),
    /// Quickly test Fate assembly programs
    Test(TestArgs),
}

#[derive(Debug)]
enum Return {
    Emulator(EmulatorError),
    Deploy(DeployError),
    Assembler(AssemblerError),
    Test,
    Ok,
}

impl Termination for Return {
    fn report(self) -> ExitCode {
        match self {
            Return::Emulator(err) => error!("{err}").emit(),
            Return::Deploy(err) => error!("{err}").emit(),
            Return::Test => {}
            Return::Assembler(AssemblerError::Assembly(errors)) => {
                for err in errors {
                    err.emit()
                }

                error!("assembly failed due to previous errors").emit();
            }
            Return::Assembler(AssemblerError::IO(err)) => err.emit(),
            Return::Ok => return ExitCode::SUCCESS,
        }
        ExitCode::FAILURE
    }
}

pub static VERBOSITY: OnceLock<Verbosity> = OnceLock::new();

fn main() -> Return {
    let cli = Args::parse();

    let verbose = match cli.verbose.log_level() {
        Some(level) => match level {
            Level::Error => Verbosity::Error,
            Level::Warn => Verbosity::Warn,
            Level::Info => Verbosity::Help,
            Level::Debug | Level::Trace => Verbosity::Info,
        },
        None => Verbosity::Quiet,
    };
    VERBOSITY
        .set(verbose)
        .expect_or_scream("verbosity should be empty");

    match cli.command {
        Command::Emulate(args) => match async_std::task::block_on(emulator::emulate(args)) {
            Ok(_) => Return::Ok,
            Err(err) => Return::Emulator(err),
        },
        Command::Deploy(args) => match deploy::deploy(args) {
            Ok(_) => Return::Ok,
            Err(err) => Return::Deploy(err),
        },
        Command::Assemble(args) => match assembler::assemble(args) {
            Ok(_) => Return::Ok,
            Err(err) => Return::Assembler(err),
        },
        Command::Test(args) => match tests::test_all(args) {
            Ok(_) => Return::Ok,
            Err(_) => Return::Test,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verbosity {
    Quiet = 0,
    Error = 1,
    Warn = 2,
    Help = 3,
    Info = 4,
}

impl PartialOrd for Verbosity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

impl Ord for Verbosity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}
