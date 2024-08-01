use clap::Args;
use clio::Input;

use crate::assembler::tests::{
    generator,
    lex::{self, Token, TokenInner},
    parse,
};
use crate::emulator::test_emulate;
use crate::{diagnostic::Diagnostic, error, spanned_error};
use crate::{Verbosity, VERBOSITY};

use std::{
    io::{stdout, Write},
    num::ParseIntError,
    thread,
    time::Duration,
};

use colored::Colorize;

#[derive(Debug, Args)]
pub struct TestArgs {
    inputs: Vec<Input>,
    #[clap(short, long, default_value = "500ms")]
    timeout: humantime::Duration,
}

pub fn test_all(args: TestArgs) -> Result<(), ()> {
    let mut handles = Vec::new();

    for input in args.inputs {
        let name = format!("{input}");

        handles.push((
            name.clone(),
            thread::spawn(move || {
                let mut output = Vec::new();

                test_file(input, args.timeout.into(), &mut output).map_err(|err| {
                    writeln!(output, "{err}").unwrap();
                    output
                })
            }),
        ));
    }

    let mut joined = Vec::new();

    for handle in handles {
        joined.push((
            handle.0,
            handle.1.join().expect("one of the test threads panicked"),
        ));
    }

    for handle in joined.iter() {
        if let Err(ref err) = handle.1 {
            println!("---- {} stdout ----", handle.0);
            stdout().write(err).unwrap();
            println!();
        }
    }

    for handle in joined {
        let status = match handle.1 {
            Ok(_) => "success".green(),
            Err(_) => "failure".red(),
        };

        println!("{} - {status}", handle.0);
    }

    Ok(())
}

#[inline]
fn emit_errors(errors: Vec<Diagnostic>, mut out: impl std::io::Write) -> Diagnostic {
    for err in errors {
        write!(out, "{err}").unwrap();
    }

    error!("unable to assemble due to previous errors")
}

#[inline]
fn bank_assert(bank: u8, name: &str, expected: Option<u8>) -> Result<(), Diagnostic> {
    if let Some(reg) = expected {
        if bank == reg {
            Ok(())
        } else {
            Err(error!(
                "register {name} does not equal expected value: {bank} != {reg}"
            ))
        }
    } else {
        Ok(())
    }
}

fn parse_expected(input: &str) -> Result<u8, ParseIntError> {
    if let Some(expected) = input.strip_prefix("0b") {
        u8::from_str_radix(expected, 2)
    } else if let Some(expected) = input.strip_prefix("0o") {
        u8::from_str_radix(expected, 8)
    } else if let Some(expected) = input.strip_prefix("0x") {
        u8::from_str_radix(expected, 16)
    } else {
        u8::from_str_radix(input, 10)
    }
}

fn test_file(
    input: Input,
    timeout: Duration,
    mut out: impl std::io::Write,
) -> Result<(), Diagnostic> {
    VERBOSITY.get_or_init(|| Verbosity::Error);

    let mut a = None;
    let mut b = None;
    let mut c = None;
    let mut d = None;
    let mut e = None;
    let mut f = None;
    let mut h = None;
    let mut l = None;

    let lexed = lex::lex(input).map_err(|errors| emit_errors(errors, &mut out))?;
    let mut run = true;

    let mut skipped = lexed.iter().filter(|tok| tok.inner != TokenInner::NewLine);
    while let Some(Token {
        span,
        inner: TokenInner::Doc(docstr),
    }) = skipped.next()
    {
        let trimmed = docstr.trim();
        if let Some(val) = trimmed.strip_prefix("a:") {
            a = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("b:") {
            b = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("c:") {
            c = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("d:") {
            d = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("e:") {
            e = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("f:") {
            f = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("h:") {
            h = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if let Some(val) = trimmed.strip_prefix("l:") {
            l = Some(parse_expected(val.trim()).map_err(|err| {
                spanned_error!(span.clone(), "unable to parse 8-bit integer: {err}")
            })?);
        } else if trimmed == "no-run" {
            run = false;
        }
    }

    let parsed = parse::parse(lexed).map_err(|errors| emit_errors(errors, &mut out))?;
    let assembled = generator::generate(parsed).map_err(|errors| emit_errors(errors, &mut out))?;

    if run {
        let bank = test_emulate(assembled.into(), timeout)
            .map_err(|_| error!("emulator exceeded timeout"))?;

        bank_assert(bank.a, "A", a)?;
        bank_assert(bank.b, "B", b)?;
        bank_assert(bank.c, "C", c)?;
        bank_assert(bank.d, "D", d)?;
        bank_assert(bank.e, "E", e)?;
        bank_assert(bank.f, "F", f)?;
        bank_assert(bank.h, "H", h)?;
        bank_assert(bank.l, "L", l)?;
    }

    Ok(())
}

#[cfg(test)]
#[test]
fn fib() {
    if let Err(err) = test_file(
        Input::new("tests/fib.asm").unwrap(),
        Duration::from_millis(250),
        stdout(),
    ) {
        err.scream();
    }
}

#[cfg(test)]
#[test]
fn arithmetic() {
    if let Err(err) = test_file(
        Input::new("tests/arithmetic.asm").unwrap(),
        Duration::from_millis(250),
        stdout(),
    ) {
        err.scream();
    }
}

#[cfg(test)]
#[test]
fn mem() {
    if let Err(err) = test_file(
        Input::new("tests/mem.asm").unwrap(),
        Duration::from_millis(250),
        stdout(),
    ) {
        err.scream()
    }
}

#[cfg(test)]
#[test]
fn comments() {
    if let Err(err) = test_file(
        Input::new("tests/comments.asm").unwrap(),
        Duration::from_millis(250),
        stdout(),
    ) {
        err.scream()
    }
}

#[cfg(test)]
#[test]
#[should_panic]
fn timeout() {
    if let Err(err) = test_file(
        Input::new("tests/timeout.asm").unwrap(),
        Duration::from_millis(250),
        stdout(),
    ) {
        err.scream()
    }
}
