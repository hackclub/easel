//! Assembles the specified file.
//!
//! Will be completed once I actually fix the assembler.

mod ascii;
mod eval;
pub mod generator;
mod include;
pub mod lex;
pub mod parse;
mod token;
pub use crate::diagnostic::Diagnostic;
use crate::error;

pub mod tests {
    pub use super::{generator, lex, parse};
}

use std::time::Instant;

use clap::Args;
use clio::{Input, Output};
use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct AssemblerArgs {
    /// CPU frequency in HZ.
    ///
    /// Assigned to the `CPU_FREQUENCY` variable.
    #[clap(short, long, default_value_t = 500_000)]
    frequency: u64,

    #[clap(value_parser, default_value = "-")]
    input: Input,
    #[clap(short, long, value_parser, default_value = "-")]
    output: Output,
}

#[derive(Debug, Error)]
pub enum AssemblerError {
    #[error("assembly failed")]
    Assembly(Errors),
    #[error(transparent)]
    IO(#[from] Diagnostic),
}

impl From<Errors> for AssemblerError {
    fn from(value: Errors) -> Self {
        AssemblerError::Assembly(value)
    }
}

pub type Errors = Vec<Diagnostic>;

pub fn assemble(mut args: AssemblerArgs) -> Result<(), AssemblerError> {
    let start = Instant::now();
    // Store the input name
    let input = format!("{}", args.input);

    let lexed = lex::lex(args.input)?;
    let parsed = parse::parse(lexed)?;
    let assembled = generator::generate(parsed)?;

    args.output
        .lock()
        .write_all(&assembled)
        .map_err(|err| error!("failed to write to output: {err}"))?;
    args.output
        .finish()
        .map_err(|err| error!("failed to finalize output: {err}"))?;

    let elapsed = start.elapsed().as_millis();
    let seconds = elapsed / 1000;
    let millis = elapsed % 1000;
    println!(
        "    {} assembling `{}` in {seconds}.{millis:03}s",
        "Finished".green().bold(),
        input.trim_matches('"')
    );

    Ok(())
}
