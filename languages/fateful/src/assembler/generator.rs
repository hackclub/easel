use std::cmp::Ordering;
use std::ops::{Deref, Range};
use std::sync::Arc;

use crate::spanned_warn;
use crate::{
    assembler::{
        eval,
        lex::{Ident, Span, Token, TokenInner},
        parse::DSeg,
        token::Immediate,
    },
    diagnostic::Reference,
    spanned_error, Token,
};

use super::{
    lex::Register,
    lex::TokenStream,
    parse::{Argument, Bracketed, CSeg, Inst, Label, Macro, Parenthesized, ParseStream, ParseTok},
    Diagnostic, Errors,
};

use std::collections::HashMap;

const IMMEDIATE_MASK: u8 = 0b0000_1000;
const ADD: u8 = 0x00;
const SUB: u8 = 0x10;
const ADC: u8 = 0x20;
const SBB: u8 = 0x30;
const NAND: u8 = 0x40;
const OR: u8 = 0x50;
const CMP: u8 = 0x60;
const MV: u8 = 0x70;
const LD: u8 = 0x80;
const ST: u8 = 0x90;
const LDA: u8 = 0xA0;
const LPM: u8 = 0xB0;
const PUSH: u8 = 0xC0;
const POP: u8 = 0xD0;
const JNZ: u8 = 0xE0;
const HALT: u8 = 0xF0;

pub struct Usable {
    pub address: u16,
    pub span: Arc<Span>,
    pub uses: usize,
}

enum Instruction {
    Add(Register, RegImm),
    Sub(Register, RegImm),
    Adc(Register, RegImm),
    Sbb(Register, RegImm),
    Nand(Register, RegImm),
    Or(Register, RegImm),
    Cmp(Register, RegImm),
    Mv(Register, RegImm),
    LdHl(Register),
    LdAddr(Register, Bracketed<TokenStream>),
    StHl(Register),
    StAddr(Bracketed<TokenStream>, Register),
    Lda(Bracketed<TokenStream>),
    LpmHl(Register),
    LpmAddr(Register, Bracketed<TokenStream>),
    Push(RegImm),
    Pop(Register),
    Jnz(RegImm),
    Halt,
}

impl Instruction {
    fn size(&self) -> u16 {
        match self {
            Instruction::Add(_, _) => 2,
            Instruction::Sub(_, _) => 2,
            Instruction::Adc(_, _) => 2,
            Instruction::Sbb(_, _) => 2,
            Instruction::Nand(_, _) => 2,
            Instruction::Or(_, _) => 2,
            Instruction::Cmp(_, _) => 2,
            Instruction::Mv(_, _) => 2,
            Instruction::LdHl(_) => 1,
            Instruction::LdAddr(_, _) => 3,
            Instruction::StHl(_) => 1,
            Instruction::StAddr(_, _) => 3,
            Instruction::Lda(_) => 3,
            Instruction::LpmHl(_) => 1,
            Instruction::LpmAddr(_, _) => 3,
            Instruction::Push(regimm) => match regimm {
                RegImm::Immediate(_) | RegImm::Expr(_) => 2,
                RegImm::Register(_) => 1,
            },
            Instruction::Pop(_) => 1,
            Instruction::Jnz(regimm) => match regimm {
                RegImm::Immediate(_) | RegImm::Expr(_) => 2,
                RegImm::Register(_) => 1,
            },
            Instruction::Halt => 1,
        }
    }

    fn compile(
        self,
        pc: u16,
        parent: &str,
        data: &mut HashMap<String, Usable>,
        labels: &mut HashMap<String, Usable>,
    ) -> Result<Bytes, Diagnostic> {
        match self {
            Instruction::Add(reg, regimm) => {
                Instruction::compile_double(ADD, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Sub(reg, regimm) => {
                Instruction::compile_double(SUB, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Adc(reg, regimm) => {
                Instruction::compile_double(ADC, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Sbb(reg, regimm) => {
                Instruction::compile_double(SBB, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Nand(reg, regimm) => {
                Instruction::compile_double(NAND, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Or(reg, regimm) => {
                Instruction::compile_double(OR, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Cmp(reg, regimm) => {
                Instruction::compile_double(CMP, reg, regimm, pc, parent, labels, data)
            }
            Instruction::Mv(reg, regimm) => {
                Instruction::compile_double(MV, reg, regimm, pc, parent, labels, data)
            }
            Instruction::LdHl(reg) => Ok(Bytes::Single([LD | reg as u8])),
            Instruction::LdAddr(reg, expr) => {
                let addr = Instruction::eval_address(expr, data)?;
                Ok(Bytes::Triple([
                    LD | IMMEDIATE_MASK | reg as u8,
                    (addr >> 8) as u8,
                    (addr & 0xFF) as u8,
                ]))
            }
            Instruction::StHl(reg) => Ok(Bytes::Single([ST | reg as u8])),
            Instruction::StAddr(expr, reg) => {
                let addr = Instruction::eval_address(expr, data)?;
                Ok(Bytes::Triple([
                    ST | IMMEDIATE_MASK | reg as u8,
                    (addr >> 8) as u8,
                    (addr & 0xFF) as u8,
                ]))
            }
            Instruction::Lda(expr) => {
                let addr = Instruction::eval_either(expr, pc, parent, labels, data)?;
                Ok(Bytes::Triple([
                    LDA | IMMEDIATE_MASK,
                    (addr >> 8) as u8,
                    (addr & 0xFF) as u8,
                ]))
            }
            Instruction::LpmHl(reg) => Ok(Bytes::Single([LPM | reg as u8])),
            Instruction::LpmAddr(reg, expr) => {
                let addr = Instruction::eval_reference(expr, pc, parent, labels)?;
                Ok(Bytes::Triple([
                    LPM | IMMEDIATE_MASK | reg as u8,
                    (addr >> 8) as u8,
                    (addr & 0xFF) as u8,
                ]))
            }
            Instruction::Push(regimm) => match regimm {
                RegImm::Register(reg) => Ok(Bytes::Single([PUSH | reg as u8])),
                RegImm::Immediate(imm) => Ok(Bytes::Double([PUSH | IMMEDIATE_MASK, imm])),
                RegImm::Expr(mut expr) => {
                    Instruction::expand_expr(&mut expr.inner, pc, parent);
                    let span = Span::same_line(&expr.open.span, &expr.close.span);
                    Ok(Bytes::Double([
                        PUSH | IMMEDIATE_MASK,
                        eval::eval_expr(&expr, labels, data)?
                            .value
                            .try_into()
                            .map_err(|_| spanned_error!(span, "immediate out of range"))?,
                    ]))
                }
            },
            Instruction::Pop(reg) => Ok(Bytes::Single([POP | reg as u8])),
            Instruction::Jnz(regimm) => match regimm {
                RegImm::Register(reg) => Ok(Bytes::Single([JNZ | reg as u8])),
                RegImm::Immediate(imm) => Ok(Bytes::Double([JNZ | IMMEDIATE_MASK, imm])),
                RegImm::Expr(mut expr) => {
                    Instruction::expand_expr(&mut expr.inner, pc, parent);
                    let span = Span::same_line(&expr.open.span, &expr.close.span);
                    Ok(Bytes::Double([
                        JNZ | IMMEDIATE_MASK,
                        eval::eval_expr(&expr, labels, data)?
                            .value
                            .try_into()
                            .map_err(|_| spanned_error!(span, "immediate out of range"))?,
                    ]))
                }
            },
            Instruction::Halt => Ok(Bytes::Single([HALT])),
        }
    }

    fn expand_expr(expr: &mut [Token], pc: u16, parent: &str) {
        for tok in expr.iter_mut() {
            if let TokenInner::Location = tok.inner {
                tok.inner = TokenInner::Immediate(pc as i128);
            } else if let TokenInner::Ident(Ident::Ident(ref mut name)) = tok.inner {
                *name = parent.to_owned() + name;
            }
        }
    }

    fn compile_double(
        instruction: u8,
        reg: Register,
        regimm: RegImm,
        pc: u16,
        parent: &str,
        labels: &mut HashMap<String, Usable>,
        data: &mut HashMap<String, Usable>,
    ) -> Result<Bytes, Diagnostic> {
        match regimm {
            RegImm::Register(second) => Ok(Bytes::Double([instruction | reg as u8, second as u8])),
            RegImm::Immediate(imm) => Ok(Bytes::Double([
                instruction | IMMEDIATE_MASK | reg as u8,
                imm,
            ])),
            RegImm::Expr(mut expr) => {
                Instruction::expand_expr(&mut expr.inner, pc, parent);

                let expr_span = Span::same_line(&expr.open.span, &expr.close.span);
                Ok(Bytes::Double([
                    instruction | IMMEDIATE_MASK | reg as u8,
                    eval::eval_expr(&expr, labels, data)?
                        .value
                        .try_into()
                        .map_err(|_| spanned_error!(expr_span, "immediate out of range"))?,
                ]))
            }
        }
    }

    fn eval_reference(
        mut expr: Bracketed<TokenStream>,
        pc: u16,
        parent: &str,
        labels: &mut HashMap<String, Usable>,
    ) -> Result<u16, Diagnostic> {
        for tok in expr.inner.iter_mut() {
            if let TokenInner::Location = tok.inner {
                tok.inner = TokenInner::Immediate(pc as i128);
            } else if let TokenInner::Ident(Ident::Ident(ref mut name)) = tok.inner {
                if name.starts_with('.') {
                    *name = parent.to_owned() + name;
                }
            } else if let TokenInner::Ident(Ident::Variable(_)) = tok.inner {
                return Err(spanned_error!(
                    tok.span.clone(),
                    "unexpected variable in program address"
                ));
            }
        }

        let addr = eval::eval_bracketed(expr, labels, false)?;
        addr.value
            .try_into()
            .map_err(|_| spanned_error!(addr.span, "address not in range"))
    }

    fn eval_address(
        mut expr: Bracketed<TokenStream>,
        variables: &mut HashMap<String, Usable>,
    ) -> Result<u16, Diagnostic> {
        for tok in expr.inner.iter_mut() {
            if let TokenInner::Location = tok.inner {
                return Err(spanned_error!(
                    tok.span.clone(),
                    "unexpected program location in memory address"
                ));
            } else if let TokenInner::Ident(Ident::Ident(_)) = tok.inner {
                return Err(spanned_error!(
                    tok.span.clone(),
                    "unexpected label in memory address"
                ));
            }
        }

        let addr = eval::eval_bracketed(expr, variables, true)?;
        addr.value
            .try_into()
            .map_err(|_| spanned_error!(addr.span, "address not in range"))
    }

    fn eval_either(
        mut expr: Bracketed<TokenStream>,
        pc: u16,
        parent: &str,
        labels: &mut HashMap<String, Usable>,
        variables: &mut HashMap<String, Usable>,
    ) -> Result<u16, Diagnostic> {
        let mut mem = None;
        let mut prog = None;

        for tok in expr.inner.iter_mut() {
            if let TokenInner::Ident(Ident::Ident(ref mut name)) = tok.inner {
                if let Some(prev) = mem {
                    return Err(Diagnostic::referencing_error(
                        tok.span.clone(),
                        "unexpected label in memory address",
                        Reference::new(prev, "interpreted as memory address due to this reference"),
                    ));
                }
                prog.get_or_insert(tok.span.clone());

                if name.starts_with('.') {
                    *name = parent.to_owned() + name;
                }
            } else if let TokenInner::Ident(Ident::Variable(_)) = tok.inner {
                if let Some(prev) = prog {
                    return Err(Diagnostic::referencing_error(
                        tok.span.clone(),
                        "unexpected variable in program address",
                        Reference::new(
                            prev,
                            "interpreted as program address due to this reference",
                        ),
                    ));
                }
                mem.get_or_insert(tok.span.clone());
            } else if let TokenInner::Location = tok.inner {
                if let Some(prev) = mem {
                    return Err(Diagnostic::referencing_error(
                        tok.span.clone(),
                        "unexpected program location in memory address",
                        Reference::new(prev, "interpreted as memory address due to this reference"),
                    ));
                }
                prog.get_or_insert(tok.span.clone());

                tok.inner = TokenInner::Immediate(pc as i128);
            }
        }

        let evaled = eval::eval_bracketed(
            expr,
            if mem.is_some() { variables } else { labels },
            mem.is_some(),
        )?;
        evaled
            .value
            .try_into()
            .map_err(|_| spanned_error!(evaled.span, "address not in range"))
    }
}

impl TryFrom<Inst> for Instruction {
    type Error = Diagnostic;

    fn try_from(value: Inst) -> Result<Self, Self::Error> {
        let mut args = value.args.into_values();
        match value.name.value.as_str() {
            "add" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Add(reg, regimm))
            }
            "sub" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Sub(reg, regimm))
            }
            "adc" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Adc(reg, regimm))
            }
            "sbb" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Sbb(reg, regimm))
            }
            "nand" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Nand(reg, regimm))
            }
            "or" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Or(reg, regimm))
            }
            "cmp" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Cmp(reg, regimm))
            }
            "mv" => {
                if args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 2 arguments, found {}",
                        args.len()
                    ));
                }

                let (reg, regimm) = pull_double(args)?;
                Ok(Instruction::Mv(reg, regimm))
            }
            "ld" => {
                if args.len() != 1 && args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 or 2 arguments, found {}",
                        args.len()
                    ));
                }

                let reg = match args.swap_remove(0) {
                    Argument::Reg(reg) => reg.inner,
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected register, found {}",
                            arg.description()
                        ))
                    }
                };

                if args.is_empty() {
                    Ok(Instruction::LdHl(reg))
                } else {
                    let addr = match args.swap_remove(0) {
                        Argument::Addr(addr) => addr,
                        arg => {
                            return Err(spanned_error!(
                                arg.span(),
                                "expected address, found {}",
                                arg.description()
                            ))
                        }
                    };

                    Ok(Instruction::LdAddr(reg, addr))
                }
            }
            "st" => match args.len() {
                1 => {
                    let reg = match args.swap_remove(0) {
                        Argument::Reg(reg) => reg.inner,
                        arg => {
                            return Err(spanned_error!(
                                arg.span(),
                                "expected register, found {}",
                                arg.description(),
                            ))
                        }
                    };

                    Ok(Instruction::StHl(reg))
                }
                2 => {
                    let addr = match args.swap_remove(0) {
                        Argument::Addr(addr) => addr,
                        arg => {
                            return Err(spanned_error!(
                                arg.span(),
                                "expected address, found {}",
                                arg.description(),
                            ))
                        }
                    };

                    let reg = match args.swap_remove(0) {
                        Argument::Reg(reg) => reg.inner,
                        arg => {
                            return Err(spanned_error!(
                                arg.span(),
                                "expected register, found {}",
                                arg.description(),
                            ))
                        }
                    };

                    Ok(Instruction::StAddr(addr, reg))
                }
                len => Err(spanned_error!(
                    value.name.span,
                    "expected 1 or 2 arguments, found {len}"
                )),
            },
            "lda" => {
                if args.len() != 1 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 argument, found {}",
                        args.len()
                    ));
                }

                let addr = match args.swap_remove(0) {
                    Argument::Addr(addr) => addr,
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected address, found {}",
                            arg.description(),
                        ))
                    }
                };

                Ok(Instruction::Lda(addr))
            }
            "lpm" => {
                if args.len() != 1 && args.len() != 2 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 or 2 arguments, found {}",
                        args.len()
                    ));
                }

                let reg = match args.swap_remove(0) {
                    Argument::Reg(reg) => reg.inner,
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected register, found {}",
                            arg.description()
                        ))
                    }
                };

                if args.is_empty() {
                    Ok(Instruction::LpmHl(reg))
                } else {
                    let addr = match args.swap_remove(0) {
                        Argument::Addr(addr) => addr,
                        arg => {
                            return Err(spanned_error!(
                                arg.span(),
                                "expected address, found {}",
                                arg.description()
                            ))
                        }
                    };

                    Ok(Instruction::LpmAddr(reg, addr))
                }
            }
            "push" => {
                if args.len() != 1 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 argument, found {}",
                        args.len()
                    ));
                }

                let regimm = match args.swap_remove(0) {
                    Argument::Reg(reg) => RegImm::Register(reg.inner),
                    Argument::Immediate(imm) => RegImm::Immediate(
                        imm.value
                            .try_into()
                            .map_err(|_| spanned_error!(imm.span, "immediate out of range"))?,
                    ),
                    Argument::Expr(expr) => RegImm::Expr(expr),
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected register or immediate, found {}",
                            arg.description()
                        ))
                    }
                };

                Ok(Instruction::Push(regimm))
            }
            "pop" => {
                if args.len() != 1 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 argument, found {}",
                        args.len()
                    ));
                }

                let reg = match args.swap_remove(0) {
                    Argument::Reg(reg) => reg.inner,
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected register, found {}",
                            arg.description()
                        ))
                    }
                };

                Ok(Instruction::Pop(reg))
            }
            "jnz" => {
                if args.len() != 1 {
                    return Err(spanned_error!(
                        value.name.span,
                        "expected 1 argument, found {}",
                        args.len()
                    ));
                }

                let regimm = match args.swap_remove(0) {
                    Argument::Reg(reg) => RegImm::Register(reg.inner),
                    Argument::Immediate(imm) => RegImm::Immediate(
                        imm.value
                            .try_into()
                            .map_err(|_| spanned_error!(imm.span, "immediate out of range"))?,
                    ),
                    Argument::Expr(expr) => RegImm::Expr(expr),
                    arg => {
                        return Err(spanned_error!(
                            arg.span(),
                            "expected register or immediate, found {}",
                            arg.description()
                        ))
                    }
                };

                Ok(Instruction::Jnz(regimm))
            }
            "halt" => match args.len() {
                0 => Ok(Instruction::Halt),
                len => Err(spanned_error!(
                    value.name.span,
                    "didn't expect any arguments, found {len}"
                )),
            },
            _ => Err(spanned_error!(value.name.span, "unknown instruction")),
        }
    }
}

/// # Panics
///
/// Panics if the arguments are not an immediate followed by a register or immediate.
/// Should be guarded by the calls to [`Instructions::matches`] in [`expand_macros`]
fn pull_double(mut args: Vec<Argument>) -> Result<(Register, RegImm), Diagnostic> {
    let reg = match args.swap_remove(0) {
        Argument::Reg(reg) => reg.inner,
        arg => {
            return Err(spanned_error!(
                arg.span(),
                "expected register, found {}",
                arg.description()
            ))
        }
    };

    let regimm = match args.swap_remove(0) {
        Argument::Reg(reg) => RegImm::Register(reg.inner),
        Argument::Immediate(imm) => RegImm::Immediate(
            imm.value
                .try_into()
                .map_err(|_| spanned_error!(imm.span, "immediate out of range"))?,
        ),
        Argument::Expr(expr) => RegImm::Expr(expr),
        arg => {
            return Err(spanned_error!(
                arg.span(),
                "expected register or immediate, found {}",
                arg.description()
            ))
        }
    };

    Ok((reg, regimm))
}

#[derive(Debug, Clone, Copy)]
enum Bytes {
    Single([u8; 1]),
    Double([u8; 2]),
    Triple([u8; 3]),
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Bytes::Single(byte) => byte.as_slice(),
            Bytes::Double(bytes) => bytes.as_slice(),
            Bytes::Triple(bytes) => bytes.as_slice(),
        }
    }
}

fn compile(
    mut stream: Vec<ExpSeg>,
    mut data: HashMap<String, Usable>,
) -> Result<[u8; 1 << 16], Errors> {
    // Pre-sort the segment stream to avoid segments placed physically
    // above segments in the source from mistakenly coliding
    stream.sort_by(|lhs, rhs| match (lhs.org.as_ref(), rhs.org.as_ref()) {
        (Some(lhs_org), Some(rhs_org)) => lhs_org.value.cmp(&rhs_org.value),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });

    let mut errors = Errors::new();
    let mut labels = HashMap::new();
    let mut ranges: Vec<Range<u16>> = Vec::new();
    let mut parent = String::new();
    let mut pc: u16 = 0;

    for segment in stream.iter() {
        pc = match segment.origin(pc) {
            Ok(origin) => origin,
            Err(err) => {
                errors.push(err);
                pc
            }
        };
        let start = pc;

        for expr in segment.instructions.iter() {
            match expr {
                ExpTok::Instruction(inst) => pc += inst.size(),
                ExpTok::Label(label) => {
                    let name = if label.name.value.starts_with('.') {
                        parent.to_owned() + &label.name.value
                    } else {
                        parent = label.name.value.to_owned();
                        label.name.value.to_owned()
                    };

                    let span = label.name.span.clone();
                    if let Some(prev) = labels.insert(
                        name,
                        Usable {
                            address: pc,
                            span: label.name.span.clone(),
                            uses: 0,
                        },
                    ) {
                        errors.push(Diagnostic::referencing_error(
                            span,
                            "duplicate label definitions",
                            Reference::new(prev.span, "previous definition found here"),
                        ));
                    }
                }
                ExpTok::Bytes(bytes) => pc += bytes.len() as u16,
            }
        }

        let segment_range = start..pc;

        for (i, range) in ranges.iter().enumerate() {
            if segment_range.start.max(range.start) < segment_range.end.min(range.end) {
                errors.push(
                    match stream[i].cseg {
                        Some(ref cseg) => Diagnostic::referencing_error(
                            // this will only be called after the first segment,
                            // and all segments after the first will have a `cseg` token
                            unsafe { segment.cseg.as_ref().unwrap_unchecked().span.clone() },
                            "data segment collision",
                            Reference::new(
                                cseg.span.clone(),
                                "overlaps with this segment")
                        )
                        .with_help("segments of the same type cannot overlap; try adjusting the origin or variable sizes"),
                        None => spanned_error!(
                            // this will only be called after the first segment,
                            // and all segments after the first will have a `cseg` token
                            unsafe { segment.cseg.as_ref().unwrap_unchecked().span.clone() },
                            "code segment collision"
                        )
                        .with_help("collides with the default segment")
                        .with_help("segments of the same type cannot overlap; try adjusting the origin or variable sizes"),
                    }
                )
            }
        }

        ranges.push(segment_range);
    }

    parent.clear();
    pc = 0;
    let mut program = [0; 1 << 16];

    for segment in stream {
        pc = match segment.origin(pc) {
            Ok(origin) => origin,
            Err(err) => {
                errors.push(err);
                pc
            }
        };

        for expr in segment.instructions {
            match expr {
                ExpTok::Instruction(inst) => {
                    let inst = match inst.compile(pc, &parent, &mut data, &mut labels) {
                        Ok(inst) => inst,
                        Err(err) => {
                            errors.push(err);
                            continue;
                        }
                    };

                    for byte in inst.into_iter() {
                        program[pc as usize] = *byte;
                        pc += 1;
                    }
                }
                ExpTok::Label(label) => {
                    if !label.name.value.contains('.') {
                        parent = label.name.value;
                    }
                }
                ExpTok::Bytes(bytes) => {
                    for byte in bytes {
                        program[pc as usize] = byte;
                        pc += 1;
                    }
                }
            }
        }
    }

    for (_, label) in labels {
        if label.uses == 0 {
            spanned_warn!(label.span, "unused label definition").emit()
        }
    }

    for (_, var) in data {
        if var.uses == 0 {
            spanned_warn!(var.span, "unused variable definition").emit()
        }
    }

    if errors.is_empty() {
        Ok(program)
    } else {
        Err(errors)
    }
}

enum RegImm {
    Immediate(u8),
    Expr(Parenthesized<TokenStream>),
    Register(Register),
}

pub fn assemble_data(mut stream: Vec<DSeg>) -> Result<HashMap<String, Usable>, Errors> {
    // Pre-sort the segment stream to avoid segments placed physically
    // above segments in the source from mistakenly coliding
    stream.sort_by(|lhs, rhs| match (lhs.org.as_ref(), rhs.org.as_ref()) {
            (Some(lhs_org), Some(rhs_org)) => lhs_org.value.cmp(&rhs_org.value),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
    });

    let mut variables = HashMap::new();
    let mut errors = Errors::new();
    let mut ranges: Vec<std::ops::Range<u16>> = Vec::new();
    let mut ptr = 0x0000;

    for segment in stream.iter() {
        let size = match segment.size() {
            Ok(s) => s,
            Err(err) => {
                errors.push(err);
                continue;
            }
        };
        let origin = match segment.org {
            Some(ref imm) => match imm
                .value
                .try_into()
                .map_err(|_| spanned_error!(imm.span.clone(), "segment origin out of range"))
            {
                Ok(org) => org,
                Err(err) => {
                    errors.push(err);
                    continue;
                }
            },
            None => ptr,
        };

        let segment_range = origin..(origin + size);

        for (i, range) in ranges.iter().enumerate() {
            if segment_range.start.max(range.start) < segment_range.end.min(range.end) {
                errors.push(
                    Diagnostic::referencing_error(
                        segment.dseg.span.clone(),
                        "data segment collision",
                        Reference::new(
                            stream[i].dseg.span.clone(),
                            "overlaps with this segment")
                    )
                    .with_help("segments of the same type cannot overlap; try adjusting the origin or variable sizes")
                )
            }
        }

        ranges.push(segment_range);

        for (name, (variable, span)) in segment.variables.iter() {
            let span = span.clone();
            if let Some(prev) = variables.insert(
                name.to_owned(),
                Usable {
                    address: ptr,
                    span: span.clone(),
                    uses: 0,
                },
            ) {
                errors.push(Diagnostic::referencing_error(
                    span,
                    "duplicate variable definition",
                    Reference::new(prev.span, "variable previously defined here"),
                ))
            }
            ptr += variable;
        }
    }

    if errors.is_empty() {
        Ok(variables)
    } else {
        Err(errors)
    }
}

fn expand_macro(inst: Inst, def: &Macro) -> Result<Vec<ParseTok>, Diagnostic> {
    let span = inst
        .args
        .fl()
        .map(|(first, last)| {
            let first_span = first.span();
            Arc::new(Span {
                line: first_span.line,
                source: first_span.source.clone(),
                range: first_span.start()..last.span().end(),
            })
        })
        .unwrap_or(inst.name.span.clone());

    def.expand(span, &inst.args.into_values())
}

fn expand_macros(code: Vec<CSeg>, macros: HashMap<String, Macro>) -> Result<Vec<ExpSeg>, Errors> {
    let mut errors = Errors::new();
    let mut segments = Vec::new();

    for mut segment in code {
        let mut position = 0;
        let mut exp = ExpSeg {
            cseg: segment.cseg,
            org: segment.org,
            instructions: Vec::new(),
        };

        while let Some(expr) = segment.tokens.get(position) {
            match expr {
                ParseTok::Instruction(inst) => match Instruction::try_from(inst.clone()) {
                    Ok(instruction) => exp.instructions.push(ExpTok::Instruction(instruction)),
                    Err(err) => match macros.get(&inst.name.value) {
                        Some(def) => match expand_macro(inst.clone(), def) {
                            Ok(expanded) => {
                                segment.tokens.splice(position..=position, expanded);
                                continue;
                            }
                            Err(_) => errors.push(err),
                        },
                        None => errors.push(err),
                    },
                },
                ParseTok::Label(lab) => exp.instructions.push(ExpTok::Label(lab.clone())),
                ParseTok::Bytes(bytes) => exp.instructions.push(ExpTok::Bytes(bytes.clone())),
            }
            position += 1;
        }

        segments.push(exp);
    }

    if errors.is_empty() {
        Ok(segments)
    } else {
        Err(errors)
    }
}

enum ExpTok {
    Instruction(Instruction),
    Label(Label),
    Bytes(Vec<u8>),
}

struct ExpSeg {
    cseg: Option<Token![@cseg]>,
    org: Option<Immediate>,
    instructions: Vec<ExpTok>,
}

impl ExpSeg {
    pub fn origin(&self, default: u16) -> Result<u16, Diagnostic> {
        self.org
            .as_ref()
            .map(|org| {
                org.value
                    .try_into()
                    .map_err(|_| spanned_error!(org.span.clone(), "segment origin out of range"))
            })
            .unwrap_or(Ok(default))
    }
}

pub fn generate(ctx: ParseStream) -> Result<[u8; 1 << 16], Errors> {
    let data = assemble_data(ctx.data)?;
    let expanded = expand_macros(ctx.code, ctx.macros)?;
    compile(expanded, data)
}
