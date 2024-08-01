//! WIP token tree parsing
//!
//! # Parsing Steps
//! 1. Expand if statements, defines, and parse macros
//! 2. Expand Macros
//! 3. Parse expressions and variables

use super::{
    eval, include,
    include::Lib,
    lex::{self, Delimeter, PreProc, Punctuation, Span, Token, TokenInner, TokenStream},
    token::{self, ClosedBracket, OpenBracket, Register, Variable},
    token::{
        ClosedBrace, ClosedParen, Ident, Immediate, LitString, MacroVariable, NewLine, OpenBrace,
        OpenParen, Ty,
    },
    Errors,
};

use crate::{
    assembler::token::Error,
    diagnostic::{Diagnostic, Reference},
    error, spanned_error, spanned_warn, Token,
};

use std::{collections::HashMap, iter, str::FromStr, sync::Arc};

use bitflags::bitflags;
use lazy_regex::regex_captures;

#[derive(Debug, Clone)]
pub struct Punctuated<T, S> {
    list: Vec<(T, S)>,
    last: Option<T>,
}

impl<T, S> Punctuated<T, S> {
    pub fn first<'a>(&'a self) -> Option<&'a T> {
        self.list
            .first()
            .map(|first| &first.0)
            .or(self.last.as_ref())
    }

    pub fn last<'a>(&'a self) -> Option<&'a T> {
        self.last.as_ref().or(self.list.last().map(|last| &last.0))
    }

    pub fn fl<'a>(&'a self) -> Option<(&'a T, &'a T)> {
        // SAFETY: since `first()` return `Some()`, `last()` must as well
        self.first()
            .map(|first| unsafe { (first, self.last().unwrap_unchecked()) })
    }

    pub fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
        Box::new(self.list.iter().map(|pair| &pair.0).chain(self.last.iter()))
    }

    pub fn into_values(self) -> Vec<T> {
        match self.last {
            Some(last) => self
                .list
                .into_iter()
                .map(|pair| pair.0)
                .chain(iter::once(last))
                .collect(),
            None => self.list.into_iter().map(|pair| pair.0).collect(),
        }
    }
}

macro_rules! punctuated {
    ($cursor:expr) => {
        punctuated!($cursor, $crate::assembler::lex::TokenInner::NewLine)
    };
    ($cursor:expr, $end:pat) => {{
        let mut list = Vec::new();
        let mut item = None;

        loop {
            match $cursor.peek() {
                Some(Token {
                    span: _,
                    inner: $end,
                })
                | None => break,
                _ => {}
            }

            match item {
                Some(it) => {
                    let seperator = $cursor.parse()?;
                    list.push((it, seperator));
                    item = None;
                }
                None => {
                    let next = $cursor.parse()?;
                    item = Some(next);
                }
            }
        }

        Ok(Punctuated { list, last: item })
    }};
}

macro_rules! wrapped {
    ($name:ident, $macro:ident, $open_token:pat => $open:ident, $close_token:pat => $close:ident, $closing:literal $(,)?) => {
        #[derive(Debug, Clone)]
        pub struct $name<T> {
            pub open: $open,
            pub inner: T,
            pub close: $close,
        }

        macro_rules! $macro {
            ($cursor:expr) => {
                (|| {
                    let open: $open = $cursor.parse()?;
                    let mut tokens = TokenStream::new();
                    let mut depth = 1;

                    while let Some(tok) = $cursor.peek() {
                        match tok.inner {
                            $open_token => depth += 1,
                            $close_token => {
                                depth -= 1;
                                if depth == 0 {
                                    let close: $close = $cursor.parse()?;
                                    return Ok($name {
                                        open,
                                        inner: tokens,
                                        close,
                                    });
                                }
                            }
                            _ => {}
                        }

                        // we know `next()` will return `Some()` since `peek()` was `Some()`
                        tokens.push(unsafe { $cursor.next().unwrap_unchecked() });
                    }

                    Err(spanned_error!(
                        open.span,
                        concat!("unclosed delimeter; expected closing `", $closing, "`")
                    ))
                })()
            };
            ($cursor:expr, $parsable:ty) => {{
                let open: $open = $cursor.parse()?;
                let inner: $parsable = $cursor.parse()?;
                let close = match $ctx.next() {
                    Some(Token {
                        span,
                        inner: $close_token,
                    }) => $close { span },
                    Some(tok) => return Err(spanned_error!(tok.span, "",)),
                    _ => {
                        return Err(spanned_error!(
                            open.span,
                            concat!("unclosed delimeter; expected closing `", $closing, "`")
                        ))
                    }
                };

                Ok($name { open, inner, close })
            }};
        }

        impl<T> std::ops::Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

wrapped!(
    Braced,
    braced,
    TokenInner::Delimeter(Delimeter::OpenBrace) => OpenBrace,
    TokenInner::Delimeter(Delimeter::ClosedBrace) => ClosedBrace,
    "}}"
);

wrapped!(
    Parenthesized,
    parenthesized,
    TokenInner::Delimeter(Delimeter::OpenParen) => OpenParen,
    TokenInner::Delimeter(Delimeter::ClosedParen) => ClosedParen,
    ")",
);

wrapped!(
    Bracketed,
    bracketed,
    TokenInner::Delimeter(Delimeter::OpenBracket) => OpenBracket,
    TokenInner::Delimeter(Delimeter::ClosedBracket) => ClosedBracket,
    "]",
);

#[derive(Debug)]
enum Segment {
    CSeg(CSeg),
    DSeg(DSeg),
}

impl Segment {
    fn org<'a>(&'a mut self) -> &mut Option<Immediate> {
        match self {
            Segment::CSeg(cseg) => &mut cseg.org,
            Segment::DSeg(dseg) => &mut dseg.org,
        }
    }
}

#[derive(Debug)]
pub struct DSeg {
    pub dseg: token::Dseg,
    pub org: Option<Immediate>,
    pub variables: HashMap<String, (u16, Arc<Span>)>,
}

impl DSeg {
    pub fn size(&self) -> Result<u16, Diagnostic> {
        let mut size: u16 = 0;

        for (_, (var_size, span)) in self.variables.iter() {
            size = size.checked_add(*var_size).ok_or_else(|| {
                spanned_error!(span.clone(), "data segment out of range")
                    .with_help("make sure your variables can be stored in less than 2^16 bytes.")
            })?;
        }

        Ok(size)
    }
}

#[derive(Debug)]
pub struct CSeg {
    pub cseg: Option<token::Cseg>,
    pub org: Option<Immediate>,
    pub tokens: Vec<ParseTok>,
}

#[derive(Debug)]
pub struct Cursor {
    stream: TokenStream,
    position: usize,
}

impl Cursor {
    pub fn new(stream: TokenStream) -> Self {
        Cursor {
            stream,
            position: 0,
        }
    }

    pub fn peek<'a>(&'a self) -> Option<&'a Token> {
        self.stream.get(self.position)
    }

    pub fn parse<R: Parsable>(&mut self) -> Result<R, Diagnostic> {
        R::parse(self)
    }

    fn skip_ignored(&mut self) {
        while matches!(
            self.peek(),
            Some(Token {
                inner: TokenInner::NewLine,
                span: _
            }) | Some(Token {
                inner: TokenInner::Doc(_),
                span: _
            })
        ) {
            self.position += 1;
        }
    }
}

impl Iterator for Cursor {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.stream.get(self.position);
        self.position += 1;
        ret.cloned()
    }
}

#[derive(Debug)]
pub struct Context {
    pub code: Vec<CSeg>,
    pub data: Vec<DSeg>,
    current_segment: Segment,
    pub defines: HashMap<String, TokenStream>,
    pub libs: HashMap<String, Lib>,
    pub macros: HashMap<String, Macro>,
    pub cursor: Cursor,
}

#[derive(Debug)]
pub enum ParseTok {
    Label(Label),
    Instruction(Inst),
    Bytes(Vec<u8>),
}

impl ParseTok {
    fn bytes_literal<T: TryFrom<i128>>(cursor: &mut Cursor) -> Result<T, Diagnostic> {
        match cursor.peek() {
            Some(Token {
                span: _,
                inner: TokenInner::Delimeter(Delimeter::OpenParen),
            }) => {
                let expr = parenthesized!(cursor)?;
                let eval = eval::eval_expr(&expr, &mut HashMap::new(), &mut HashMap::new())?;

                eval.value
                    .try_into()
                    .map_err(|_| spanned_error!(eval.span, "byte literal out of range"))
            }
            Some(Token {
                span,
                inner: TokenInner::Immediate(imm),
            }) => (*imm)
                .try_into()
                .map_err(|_| spanned_error!(span.clone(), "byte literal out of range")),
            Some(tok) => Err(spanned_error!(
                tok.span.clone(),
                "expected byte literal, found {}",
                tok.inner.description()
            )),
            None => Err(error!("expected byte literal, found `eof`")),
        }
    }

    fn argument_bytes_literal<T: TryFrom<i128>>(argument: &Argument) -> Result<T, Diagnostic> {
        match argument {
            Argument::Immediate(imm) => imm
                .value
                .try_into()
                .map_err(|_| error!("immediate out of range")),
            Argument::Expr(expr) => {
                let eval = eval::eval_expr(expr, &mut HashMap::new(), &mut HashMap::new())?;
                eval.value
                    .try_into()
                    .map_err(|_| spanned_error!(eval.span, "byte literal out of range"))
            }
            _ => {
                return Err(spanned_error!(
                    argument.span(),
                    "expected literal, found {}",
                    argument.description()
                ))
            }
        }
    }
}

impl Parsable for ParseTok {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        match cursor.peek() {
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Str)),
            }) => {
                cursor.position += 1;
                let val: LitString = cursor.parse()?;
                let _: NewLine = cursor.parse()?;

                Ok(ParseTok::Bytes(val.value.into_bytes()))
            }
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Byte)),
            }) => {
                cursor.position += 1;
                let byte: u8 = ParseTok::bytes_literal(cursor)?;
                Ok(ParseTok::Bytes(vec![byte]))
            }
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Double)),
            }) => {
                cursor.position += 1;
                let bytes: u16 = ParseTok::bytes_literal(cursor)?;
                Ok(ParseTok::Bytes(bytes.to_be_bytes().to_vec()))
            }
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Quad)),
            }) => {
                cursor.position += 1;
                let bytes: u32 = ParseTok::bytes_literal(cursor)?;
                Ok(ParseTok::Bytes(bytes.to_be_bytes().to_vec()))
            }
            _ => {
                let name: Ident = cursor.parse()?;

                if let Some(Token {
                    span: _,
                    inner: TokenInner::Punctuation(Punctuation::Colon),
                }) = cursor.peek()
                {
                    Ok(ParseTok::Label(Label {
                        name,
                        colon: cursor.parse()?,
                    }))
                } else {
                    let args = punctuated!(cursor)?;
                    // we know there's a newline at the end, so we can just skip it
                    cursor.position += 1;

                    Ok(ParseTok::Instruction(Inst { name, args }))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inst {
    pub name: Ident,
    pub args: Punctuated<Argument, Token![,]>,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: Ident,
    pub colon: Token![:],
}

#[derive(Debug)]
pub struct ParseStream {
    pub code: Vec<CSeg>,
    pub data: Vec<DSeg>,
    pub macros: HashMap<String, Macro>,
}

pub fn parse(mut stream: TokenStream) -> Result<ParseStream, Errors> {
    let mut errors = Vec::new();

    let s = match include::include_builtins() {
        Ok(mut macros) => {
            macros.append(&mut stream);
            macros
        }
        Err(mut errs) => {
            errors.append(&mut errs);
            stream
        }
    };

    let mut ctx = Context {
        code: Vec::new(),
        data: Vec::new(),
        current_segment: Segment::CSeg(CSeg {
            org: None,
            cseg: None,
            tokens: Vec::new(),
        }),
        defines: HashMap::new(),
        libs: HashMap::new(),
        macros: HashMap::new(),
        cursor: Cursor::new(s),
    };

    while let Some(tok) = ctx.cursor.peek().cloned() {
        if let Err(mut err) = expand_preproc(tok, &mut ctx) {
            errors.append(&mut err);
            return Err(errors);
        }
    }

    ctx.cursor.position = 0;
    ctx.cursor.skip_ignored();

    while let Some(tok) = ctx.cursor.peek() {
        if let TokenInner::Ident(lex::Ident::PreProc(PreProc::Cseg)) = tok.inner {
            let mut segment = Segment::CSeg(CSeg {
                cseg: Some(
                    ctx.cursor
                        .parse()
                        .map_err(|err| Into::<Errors>::into(err))?,
                ),
                org: None,
                tokens: Vec::new(),
            });

            std::mem::swap(&mut segment, &mut ctx.current_segment);

            match segment {
                Segment::CSeg(cseg) => ctx.code.push(cseg),
                Segment::DSeg(dseg) => ctx.data.push(dseg),
            }

            ctx.cursor.position += 1;
        } else if let TokenInner::Ident(lex::Ident::PreProc(PreProc::Dseg)) = tok.inner {
            let mut segment = Segment::DSeg(DSeg {
                dseg: ctx
                    .cursor
                    .parse()
                    .map_err(|err| Into::<Errors>::into(err))?,
                org: None,
                variables: HashMap::new(),
            });

            std::mem::swap(&mut segment, &mut ctx.current_segment);

            match segment {
                Segment::CSeg(cseg) => ctx.code.push(cseg),
                Segment::DSeg(dseg) => ctx.data.push(dseg),
            }

            ctx.cursor.position += 1;
        } else if let TokenInner::Ident(lex::Ident::PreProc(PreProc::Org)) = tok.inner {
            let origin: Org = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;
            if let Some(org) = ctx.current_segment.org() {
                return Err(Diagnostic::referencing_error(
                    origin.span,
                    "duplicate definitions of origin",
                    Reference::new(org.span.clone(), "origin originally defined here"),
                )
                .with_help("`@org` can only be used once per section")
                .into());
            } else {
                *ctx.current_segment.org() = Some(origin.address);
            }
        } else {
            match ctx.current_segment {
                Segment::CSeg(ref mut cseg) => match ctx.cursor.parse() {
                    Ok(exp) => cseg.tokens.push(exp),
                    Err(err) => errors.push(err),
                },
                Segment::DSeg(ref mut dseg) => {
                    let var: VariableDef = match ctx.cursor.parse() {
                        Ok(var) => var,
                        Err(err) => {
                            errors.push(err);
                            continue;
                        }
                    };

                    let span = var.name.span.clone();
                    if let Some((_, prev_span)) = dseg
                        .variables
                        .insert(var.name.value, (var.size, var.name.span))
                    {
                        errors.push(Diagnostic::referencing_error(
                            span,
                            "duplicate variable definition",
                            Reference::new(prev_span, "variable previously defined here"),
                        ));
                    }
                }
            }
        }

        ctx.cursor.skip_ignored();
    }

    match ctx.current_segment {
        Segment::CSeg(cseg) => ctx.code.push(cseg),
        Segment::DSeg(dseg) => ctx.data.push(dseg),
    }

    if !errors.is_empty() {
        return Err(errors);
    } else {
        Ok(ParseStream {
            code: ctx.code,
            data: ctx.data,
            macros: ctx.macros,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Argument {
    Reg(Register),
    Immediate(Immediate),
    Addr(Bracketed<TokenStream>),
    Expr(Parenthesized<TokenStream>),
    Ident(Ident),
    Str(LitString),
}

impl Argument {
    pub fn description(&self) -> &'static str {
        match self {
            Argument::Reg(_) => "register",
            Argument::Immediate(_) => "immediate",
            Argument::Addr(_) => "address",
            Argument::Expr(_) => "expression",
            Argument::Ident(_) => "identifier",
            Argument::Str(_) => "string literal",
        }
    }

    pub fn span(&self) -> Arc<Span> {
        match self {
            Argument::Reg(reg) => reg.span.clone(),
            Argument::Immediate(imm) => imm.span.clone(),
            Argument::Ident(ident) => ident.span.clone(),
            Argument::Str(string) => string.span.clone(),
            Argument::Addr(addr) => Arc::new(Span {
                line: addr.open.span.line,
                range: addr.open.span.start()..addr.close.span.end(),
                source: addr.open.span.source.clone(),
            }),
            Argument::Expr(expr) => Arc::new(Span {
                line: expr.open.span.line,
                range: expr.open.span.start()..expr.close.span.end(),
                source: expr.open.span.source.clone(),
            }),
        }
    }
}

impl Parsable for Argument {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        match cursor.peek() {
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::Register(_)),
            }) => Ok(Argument::Reg(cursor.parse()?)),
            Some(Token {
                span: _,
                inner: TokenInner::Immediate(_),
            }) => Ok(Argument::Immediate(cursor.parse()?)),
            Some(Token {
                span: _,
                inner: TokenInner::Delimeter(Delimeter::OpenBracket),
            }) => Ok(Argument::Addr(bracketed!(cursor)?)),
            Some(Token {
                span: _,
                inner: TokenInner::Delimeter(Delimeter::OpenParen),
            }) => Ok(Argument::Expr(parenthesized!(cursor)?)),
            Some(Token {
                span: _,
                inner: TokenInner::String(_),
            }) => Ok(Argument::Str(cursor.parse()?)),
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::Ident(_)),
            }) => Ok(Argument::Ident(cursor.parse()?)),
            Some(_) => {
                // SAFETY: Since `peek()` returned `Some`, we know `next()` will as well
                let next = unsafe { cursor.next().unwrap_unchecked() };
                Err(spanned_error!(
                    next.span,
                    "expected argument, found {}",
                    next.inner.description()
                ))
            }
            None => Err(error!("expected argument, found `eof`")),
        }
    }
}

fn expand_preproc(peek: Token, ctx: &mut Context) -> Result<(), Errors> {
    use TokenInner as TI;
    match peek.inner {
        TI::Ident(lex::Ident::PreProc(PreProc::Define)) => {
            let start = ctx.cursor.position;
            let def: Define = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;

            ctx.cursor.stream.drain(start..ctx.cursor.position);
            ctx.cursor.position = start;

            ctx.defines.insert(def.name, def.value);
        }
        TI::Ident(lex::Ident::PreProc(PreProc::UnDef)) => {
            let start = ctx.cursor.position;
            ctx.cursor.position += 1;

            let ident: Ident = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;
            if let None = ctx.defines.remove(&ident.value) {
                spanned_warn!(ident.span, "define not found").emit();
            }

            ctx.cursor.stream.drain(start..ctx.cursor.position);
            ctx.cursor.position = start;
        }
        TI::Ident(lex::Ident::PreProc(PreProc::If)) => {
            eval_if(ctx).map_err(|err| Into::<Errors>::into(err))?
        }
        TI::Ident(lex::Ident::PreProc(PreProc::IfDef)) => {
            eval_if_def(ctx, false).map_err(|err| Into::<Errors>::into(err))?
        }
        TI::Ident(lex::Ident::PreProc(PreProc::IfNDef)) => {
            eval_if_def(ctx, true).map_err(|err| Into::<Errors>::into(err))?
        }
        TI::Ident(lex::Ident::PreProc(PreProc::Macro)) => {
            let start = ctx.cursor.position;
            let mac: Macro = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;

            ctx.cursor.stream.drain(start..ctx.cursor.position);
            ctx.cursor.position = start;

            let name = mac.name.clone();
            if let Some(first) = ctx.macros.insert(mac.name.value.to_owned(), mac) {
                return Err(Diagnostic::referencing_error(
                    name.span,
                    format!("duplicate definitions of macro `{}`", name.value),
                    Reference::new(first.name.span, "macro originally defined here"),
                )
                .into());
            }
        }
        TI::Ident(lex::Ident::PreProc(PreProc::Include)) => {
            let start = ctx.cursor.position;
            ctx.cursor.position += 1;
            let path = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;

            let tokens: TokenStream =
                include::include(path, &mut ctx.libs).map_err(|err| Into::<Errors>::into(err))?;
            ctx.cursor.stream.splice(start..ctx.cursor.position, tokens);

            ctx.cursor.position = start;
        }
        TI::Ident(lex::Ident::PreProc(PreProc::Error)) => {
            let error: Error = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;
            let str: LitString = ctx
                .cursor
                .parse()
                .map_err(|err| Into::<Errors>::into(err))?;
            return Err(vec![spanned_error!(error.span, "{}", str.value)]);
        }
        TI::Ident(lex::Ident::Ident(value)) => {
            if let Some(def) = ctx.defines.get(&value) {
                ctx.cursor
                    .stream
                    .splice(ctx.cursor.position..=ctx.cursor.position, def.clone());
            } else {
                ctx.cursor.position += 1;
            }
        }
        TI::Doc(ref doc_str) => {
            ctx.cursor.position += 1;

            if let Some((_whole, name, source)) =
                regex_captures!(r"(\s*[._a-zA-Z][._a-zA-Z0-9]*\s*)=(.*)", doc_str)
            {
                let comment_start = 3 + peek.span.start();
                let name_start = comment_start + (name.len() - name.trim_start().len());
                let name_end = comment_start + name.trim_end().len();
                let name_span = Arc::new(Span {
                    range: name_start..name_end,
                    ..(*peek.span).clone()
                });

                let source_start =
                    comment_start + name.len() + 1 + (source.len() - source.trim_start().len());
                let source_end = source_start + source.trim_end().len() - 1;

                if let Some(prev) = ctx.libs.insert(
                    name.trim().to_owned(),
                    Lib::new(
                        source.trim().to_owned(),
                        name_span.clone(),
                        Span {
                            range: source_start..source_end,
                            ..(*peek.span).clone()
                        }
                        .into(),
                    ),
                ) {
                    if prev != source {
                        Diagnostic::referencing_warning(
                            name_span,
                            "import redefined",
                            Reference::new(prev.name_span, "previous defintion here"),
                        )
                        .emit();
                    }
                }
            }
        }
        _ => ctx.cursor.position += 1,
    }

    Ok(())
}

fn eval_if_def(ctx: &mut Context, ndef: bool) -> Result<(), Diagnostic> {
    let start = ctx.cursor.position;
    let if_def_span = ctx
        .cursor
        .next()
        .ok_or_else(|| error!("`parse::eval_if_def` called with no tokens").as_bug())?
        .span;
    let def: Ident = ctx.cursor.parse()?;
    let eval = ctx.defines.contains_key(&def.value);

    expand_if(ctx, start, if_def_span, if ndef { !eval } else { eval })
}

fn eval_if(ctx: &mut Context) -> Result<(), Diagnostic> {
    let start = ctx.cursor.position;
    let if_span = ctx
        .cursor
        .next()
        .ok_or_else(|| error!("`parse::eval_if` called without tokens").as_bug())?
        .span;

    let eval = if_expr(ctx)?;
    expand_if(ctx, start, if_span, eval)
}

fn expand_if(
    ctx: &mut Context,
    start: usize,
    if_span: Arc<Span>,
    eval: bool,
) -> Result<(), Diagnostic> {
    let mut depth = 1;
    let mut out = Vec::new();

    use TokenInner as TI;
    while let Some(tok) = ctx.cursor.peek() {
        match tok.inner {
            TI::Ident(lex::Ident::PreProc(PreProc::If))
            | TI::Ident(lex::Ident::PreProc(PreProc::IfDef))
            | TI::Ident(lex::Ident::PreProc(PreProc::IfNDef)) => {
                ctx.cursor.position += 1;
                depth += 1;
            }
            TI::Ident(lex::Ident::PreProc(PreProc::EndIf)) => {
                ctx.cursor.position += 1;
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            TI::Ident(lex::Ident::PreProc(PreProc::ElIf)) => {
                if depth == 1 {
                    if eval {
                        end_if(ctx, if_span.clone())?;
                        depth -= 1;
                        break;
                    } else {
                        return eval_if(ctx);
                    }
                }
            }
            TI::Ident(lex::Ident::PreProc(PreProc::Else)) => {
                if depth == 1 {
                    if eval {
                        end_if(ctx, if_span.clone())?;
                        depth -= 1;
                        break;
                    } else {
                        while let Some(more) = ctx.cursor.next() {
                            if matches!(more.inner, TI::Ident(lex::Ident::PreProc(PreProc::EndIf)))
                            {
                                break;
                            }
                        }
                        return Err(error!("unmatched"));
                    }
                }
            }
            _ => {
                if eval {
                    // we know we will recieve `Some()` from `next()`,
                    // since we recieved `Some()` from `peek()`.
                    out.push(unsafe { ctx.cursor.next().unwrap_unchecked() });
                } else {
                    ctx.cursor.position += 1;
                }
            }
        }
    }

    if depth != 0 {
        return Err(spanned_error!(
            if_span,
            "unclosed if expression; expected `@endif`, found `eof`"
        ));
    }

    ctx.cursor.stream.splice(start..ctx.cursor.position, out);
    ctx.cursor.position = start;

    Ok(())
}

fn if_expr(ctx: &mut Context) -> Result<bool, Diagnostic> {
    let start = ctx.cursor.position;

    let end = ctx
        .cursor
        .position(|tok| tok.inner == TokenInner::NewLine)
        .ok_or_else(|| {
            spanned_error!(
                ctx.cursor.stream[ctx.cursor.stream.len() - 1].span.clone(),
                "expected newline after `@if` expression, found `EOF`"
            )
        })?;

    if end == 0 {
        return Err(spanned_error!(
            ctx.cursor.stream[start].span.clone(),
            "expected expression, found newline"
        ));
    }

    let eval = eval::eval_preproc(&ctx.cursor.stream[start..(start + end)], &ctx.defines)?;

    Ok(eval > 0)
}

fn end_if(ctx: &mut Context, err_span: Arc<Span>) -> Result<(), Diagnostic> {
    let mut depth = 1;

    while let Some(tok) = ctx.cursor.next() {
        use TokenInner as TI;
        match tok.inner {
            TI::Ident(lex::Ident::PreProc(PreProc::If)) => depth += 1,
            TI::Ident(lex::Ident::PreProc(PreProc::IfDef)) => depth += 1,
            TI::Ident(lex::Ident::PreProc(PreProc::IfNDef)) => depth += 1,
            TI::Ident(lex::Ident::PreProc(PreProc::EndIf)) => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            }
            _ => {}
        }
    }

    Err(spanned_error!(
        err_span,
        "unclosed if expression; expected `@endif`, found `eof`"
    ))
}

pub trait Parsable: Sized {
    fn parse(ctx: &mut Cursor) -> Result<Self, Diagnostic>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Org {
    span: Arc<Span>,
    address: Immediate,
}

impl Parsable for Org {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let org: Token![@org] = cursor.parse()?;
        let addr: Immediate = cursor.parse()?;

        Ok(Org {
            span: Arc::new(Span {
                line: org.span.line,
                range: org.span.range.start..addr.span.range.end,
                source: org.span.source.clone(),
            }),
            address: addr,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Define {
    pub name: String,
    pub value: TokenStream,
}

impl Parsable for Define {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let _def: Token![@define] = cursor.parse()?;
        let name: Ident = cursor.parse()?;
        let assignment: TokenStream = cursor.parse()?;

        Ok(Define {
            name: name.value,
            value: assignment,
        })
    }
}

impl FromStr for Define {
    type Err = Diagnostic;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let pos = match trimmed.find("=") {
            Some(pos) => pos,
            None => {
                return Ok(Define {
                    name: s.to_owned(),
                    value: Vec::new(),
                })
            }
        };

        let name = s[..pos].to_owned();
        if name.contains(' ') {
            return Err(spanned_error!(
                Span {
                    source: lex::Source::String {
                        name: None,
                        source: Arc::new(trimmed.to_owned())
                    },
                    line: 0,
                    range: 0..trimmed.len(),
                }
                .into(),
                "define name cannot contain whitespace",
            ));
        }

        let lexed = lex::lex_string(None, &trimmed[(pos + 1)..]);
        let value = match lexed {
            Ok(l) => l,
            Err(errors) => {
                for err in errors {
                    err.emit();
                }
                error!("Unable to lex define due to previous errors").scream();
            }
        };

        Ok(Define { name, value })
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Types: u8 {
        const REG = 0b0000_0001;
        const ADDR = 0b0000_0010;
        const LABEL = 0b0000_0100;
        const STR = 0b0000_1000;
        const IMM = 0b0001_0000;
        const IDENT = 0b0010_0000;
    }
}

impl From<Vec<Ty>> for Types {
    fn from(value: Vec<Ty>) -> Self {
        let mut types = Types::empty();

        for ty in value {
            types.insert(match ty.ty {
                lex::Ty::Reg => Types::REG,
                lex::Ty::Addr => Types::ADDR,
                lex::Ty::Label => Types::LABEL,
                lex::Ty::Str => Types::STR,
                lex::Ty::Imm => Types::IMM,
                lex::Ty::Ident => Types::IDENT,
                lex::Ty::Any => Types::all(),
            })
        }

        types
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: MacroVariable,
    types: Types,
}

impl Parameter {
    fn fits(&self, arg: &Argument) -> bool {
        match arg {
            Argument::Str(_) => self.types.contains(Types::STR),
            Argument::Immediate(_) | Argument::Expr(_) => self.types.contains(Types::IMM),
            Argument::Reg(_) => self.types.contains(Types::REG),
            Argument::Ident(_) => self.types.contains(Types::IDENT),
            Argument::Addr(expr) => {
                let mut types = Types::LABEL | Types::ADDR;

                for tok in expr.inner.iter() {
                    match tok.inner {
                        TokenInner::Ident(lex::Ident::Variable(_)) => types.remove(Types::LABEL),
                        TokenInner::Ident(lex::Ident::Ident(_)) => types.remove(Types::ADDR),
                        _ => {}
                    }
                }

                self.types.intersects(types)
            }
        }
    }
}

#[derive(Debug)]
pub struct MacroDef {
    parameters: Vec<Parameter>,
    expansion: Braced<TokenStream>,
}

impl MacroDef {
    pub fn fits(&self, tokens: &[Argument]) -> bool {
        if self.parameters.len() != tokens.len() {
            return false;
        }

        self.parameters
            .iter()
            .zip(tokens)
            .all(|(param, token)| param.fits(token))
    }

    /// Must make sure that the provided parameters match this rule with [`MacroDef::fits`]
    pub fn expand(&self, parameters: &[Argument]) -> Result<Vec<ParseTok>, Diagnostic> {
        let mut expanded = Vec::new();

        let parameters: HashMap<String, &Argument> = HashMap::from_iter(
            self.parameters
                .iter()
                .map(|p| p.name.name.to_owned())
                .zip(parameters),
        );

        let mut cursor = Cursor {
            stream: self.expansion.inner.clone(),
            position: 0,
        };

        cursor.skip_ignored();

        while let Some(tok) = cursor.peek() {
            match tok.inner {
                TokenInner::Ident(lex::Ident::PreProc(PreProc::Str)) => {
                    cursor.position += 1;
                    match cursor.peek() {
                        Some(Token {
                            span,
                            inner: TokenInner::Ident(lex::Ident::MacroVariable(var)),
                        }) => match parameters.get(var) {
                            Some(Argument::Str(str)) => {
                                expanded.push(ParseTok::Bytes(str.value.clone().into_bytes()));

                                cursor.position += 1;
                                let _: NewLine = cursor.parse()?;
                            }
                            Some(arg) => {
                                return Err(spanned_error!(
                                    arg.span(),
                                    "expected string literal, found {}",
                                    arg.description(),
                                ))
                            }
                            None => {
                                return Err(spanned_error!(
                                    span.clone(),
                                    "macro variable not found in scope",
                                ))
                            }
                        },
                        _ => {
                            let val: LitString = cursor.parse()?;
                            let _: NewLine = cursor.parse()?;

                            expanded.push(ParseTok::Bytes(val.value.into_bytes()))
                        }
                    }
                }
                TokenInner::Ident(lex::Ident::PreProc(PreProc::Byte)) => {
                    cursor.position += 1;
                    match cursor.peek() {
                        Some(Token {
                            span,
                            inner: TokenInner::Ident(lex::Ident::MacroVariable(var)),
                        }) => match parameters.get(var) {
                            Some(arg) => {
                                let byte: u8 = ParseTok::argument_bytes_literal(arg)?;
                                expanded.push(ParseTok::Bytes(vec![byte]));
                            }
                            None => {
                                return Err(spanned_error!(
                                    span.clone(),
                                    "macro variable not found in scope"
                                ))
                            }
                        },
                        _ => {
                            let byte: u8 = ParseTok::bytes_literal(&mut cursor)?;
                            expanded.push(ParseTok::Bytes(vec![byte]));
                        }
                    }
                }
                TokenInner::Ident(lex::Ident::PreProc(PreProc::Double)) => {
                    cursor.position += 1;
                    match cursor.peek() {
                        Some(Token {
                            span,
                            inner: TokenInner::Ident(lex::Ident::MacroVariable(var)),
                        }) => match parameters.get(var) {
                            Some(arg) => {
                                let bytes: u16 = ParseTok::argument_bytes_literal(arg)?;
                                expanded.push(ParseTok::Bytes(bytes.to_be_bytes().to_vec()));
                            }
                            None => {
                                return Err(spanned_error!(
                                    span.clone(),
                                    "macro variable not found in scope"
                                ))
                            }
                        },
                        _ => {
                            let bytes: u16 = ParseTok::bytes_literal(&mut cursor)?;
                            expanded.push(ParseTok::Bytes(bytes.to_be_bytes().to_vec()));
                        }
                    }
                }
                TokenInner::Ident(lex::Ident::PreProc(PreProc::Quad)) => {
                    cursor.position += 1;
                    match cursor.peek() {
                        Some(Token {
                            span,
                            inner: TokenInner::Ident(lex::Ident::MacroVariable(var)),
                        }) => match parameters.get(var) {
                            Some(arg) => {
                                let bytes: u32 = ParseTok::argument_bytes_literal(arg)?;
                                expanded.push(ParseTok::Bytes(bytes.to_be_bytes().to_vec()));
                            }
                            None => {
                                return Err(spanned_error!(
                                    span.clone(),
                                    "macro variable not found in scope"
                                ))
                            }
                        },
                        _ => {
                            let bytes: u32 = ParseTok::bytes_literal(&mut cursor)?;
                            expanded.push(ParseTok::Bytes(bytes.to_be_bytes().to_vec()));
                        }
                    }
                }
                _ => {
                    let name: Ident = match cursor.peek() {
                        Some(Token {
                            span,
                            inner: TokenInner::Ident(lex::Ident::MacroVariable(var)),
                        }) => match parameters.get(var) {
                            Some(Argument::Ident(ident)) => Ok(ident.clone()),
                            Some(arg) => {
                                return Err(spanned_error!(
                                    span.clone(),
                                    "expected identifier, found {}",
                                    arg.description()
                                ))
                            }
                            None => Err(spanned_error!(
                                span.clone(),
                                "macro variable not found in scope"
                            )),
                        },
                        _ => cursor.parse(),
                    }?;

                    if let Some(Token {
                        span: _,
                        inner: TokenInner::Punctuation(Punctuation::Colon),
                    }) = cursor.peek()
                    {
                        expanded.push(ParseTok::Label(Label {
                            name,
                            colon: cursor.parse()?,
                        }))
                    } else {
                        let args = MacroDef::replace_punctuated(&mut cursor, &parameters)?;

                        expanded.push(ParseTok::Instruction(Inst { name, args }))
                    }
                }
            }

            cursor.skip_ignored();
        }

        Ok(expanded)
    }

    fn replace_punctuated(
        cursor: &mut Cursor,
        parameters: &HashMap<String, &Argument>,
    ) -> Result<Punctuated<Argument, Token![,]>, Diagnostic> {
        let mut arguments = Vec::new();
        let mut item = None;

        loop {
            match cursor.next() {
                Some(Token {
                    span: _,
                    inner: TokenInner::NewLine,
                })
                | None => break,
                Some(tok) => match item {
                    Some(it) => {
                        cursor.position -= 1;
                        let seperator: Token![,] = cursor.parse()?;
                        arguments.push((it, seperator));
                        item = None;
                    }
                    None => {
                        let mut next = match tok.inner {
                            TokenInner::Ident(lex::Ident::MacroVariable(ref var)) => {
                                (*parameters.get(var).ok_or_else(|| {
                                    spanned_error!(tok.span, "macro variable not recognized")
                                })?)
                                .clone()
                            }
                            _ => {
                                cursor.position -= 1;
                                cursor.parse()?
                            }
                        };

                        match next {
                            Argument::Expr(ref mut parenthesized) => {
                                Self::replace_stream(&mut parenthesized.inner, parameters)?;
                            }
                            Argument::Addr(ref mut bracketed) => {
                                Self::replace_stream(&mut bracketed.inner, parameters)?;
                            }
                            _ => {}
                        }

                        item = Some(next);
                    }
                },
            }
        }

        Ok(Punctuated {
            list: arguments,
            last: item,
        })
    }

    fn replace_stream(
        stream: &mut TokenStream,
        parameters: &HashMap<String, &Argument>,
    ) -> Result<(), Diagnostic> {
        for i in 0..stream.len() {
            if let TokenInner::Ident(lex::Ident::MacroVariable(ref var)) = stream[i].inner {
                match *parameters.get(var).ok_or_else(|| {
                    spanned_error!(stream[i].span.clone(), "macro variable not recognized")
                })? {
                    Argument::Str(arg) => {
                        stream[i].inner = TokenInner::String(arg.value.clone());
                    }
                    Argument::Reg(reg) => {
                        stream[i].inner = TokenInner::Ident(lex::Ident::Register(reg.inner));
                    }
                    Argument::Immediate(imm) => {
                        stream[i].inner = TokenInner::Immediate(imm.value);
                    }
                    Argument::Ident(ident) => {
                        stream[i].inner = TokenInner::Ident(lex::Ident::Ident(ident.value.clone()));
                    }
                    Argument::Expr(expr) => {
                        stream.splice(
                            i..=i,
                            std::iter::once(Token {
                                span: expr.open.span.clone(),
                                inner: TokenInner::Delimeter(Delimeter::OpenParen),
                            })
                            .chain(expr.inner.clone())
                            .chain(std::iter::once(Token {
                                span: expr.close.span.clone(),
                                inner: TokenInner::Delimeter(Delimeter::ClosedParen),
                            })),
                        );
                    }
                    Argument::Addr(addr) => {
                        stream.splice(
                            i..=i,
                            std::iter::once(Token {
                                span: addr.open.span.clone(),
                                inner: TokenInner::Delimeter(Delimeter::OpenBracket),
                            })
                            .chain(addr.inner.clone())
                            .chain(std::iter::once(Token {
                                span: addr.close.span.clone(),
                                inner: TokenInner::Delimeter(Delimeter::ClosedBracket),
                            })),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_inputs(cursor: &mut Cursor) -> Result<Vec<Parameter>, Diagnostic> {
        let paren: OpenParen = cursor.parse()?;

        let mut params: Vec<Parameter> = Vec::new();
        if matches!(
            cursor.peek(),
            Some(Token {
                span: _,
                inner: TokenInner::Delimeter(Delimeter::ClosedParen)
            })
        ) {
            cursor.position += 1;
            return Ok(params);
        }

        while cursor.peek().is_some() {
            let var: MacroVariable = cursor.parse()?;
            for param in params.iter() {
                if param.name.name == var.name {
                    return Err(spanned_error!(
                        var.span,
                        "duplicate parameter `{}`",
                        var.name
                    ));
                }
            }
            let _seperator: Token![:] = cursor.parse()?;
            let mut types = vec![cursor.parse()?];

            while let Some(tok) = cursor.peek() {
                match tok.inner {
                    TokenInner::Delimeter(Delimeter::ClosedParen) => {
                        let _close: ClosedParen = cursor.parse()?;

                        params.push(Parameter {
                            name: var,
                            types: types.into(),
                        });

                        return Ok(params);
                    }
                    TokenInner::Punctuation(Punctuation::Comma) => {
                        cursor.position += 1;
                        break;
                    }
                    _ => {
                        let _or: Token![|] = cursor.parse()?;
                        let ty: Ty = cursor.parse()?;
                        types.push(ty);
                    }
                }
            }

            params.push(Parameter {
                name: var,
                types: types.into(),
            })
        }

        Err(spanned_error!(
            paren.span,
            "unmatched delimeter; expected closing `)`"
        ))
    }
}

impl Parsable for MacroDef {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let inputs = Self::parse_inputs(cursor)?;
        let expansion: Braced<TokenStream> = braced!(cursor)?;

        Ok(MacroDef {
            parameters: inputs,
            expansion,
        })
    }
}

#[derive(Debug)]
pub struct Macro {
    name: Ident,
    rules: Vec<MacroDef>,
}

impl Macro {
    pub fn expand(
        &self,
        span: Arc<Span>,
        parameters: &[Argument],
    ) -> Result<Vec<ParseTok>, Diagnostic> {
        let rule = self
            .rules
            .iter()
            .find(|def| def.fits(&parameters))
            .ok_or_else(|| spanned_error!(span, "no rules matched these arguments"))?;
        rule.expand(&parameters)
    }
}

impl Parsable for Macro {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let proc: Token![@macro] = cursor.parse()?;
        let name: Ident = cursor.parse()?;

        let mut rules = Vec::new();

        match cursor.peek() {
            Some(Token {
                inner: TokenInner::Delimeter(Delimeter::OpenParen),
                span: _,
            }) => {
                rules.push(cursor.parse()?);
                Ok(Macro { name, rules })
            }
            Some(Token {
                inner: TokenInner::Delimeter(Delimeter::OpenBrace),
                span: _,
            }) => {
                let brace: OpenBrace = cursor.parse()?;
                cursor.skip_ignored();

                while let Some(tok) = cursor.peek() {
                    use TokenInner as TI;
                    match tok.inner {
                        TI::Delimeter(Delimeter::OpenParen) => rules.push(cursor.parse()?),
                        TI::Delimeter(Delimeter::ClosedBrace) => {
                            let _close: ClosedBrace = cursor.parse()?;
                            let _nl: NewLine = cursor.parse()?;
                            return Ok(Macro { name, rules });
                        }
                        TI::NewLine => cursor.position += 1,
                        _ => {
                            return Err(spanned_error!(
                                tok.span.clone(),
                                "expected start of rule definition or end of macro, found {}",
                                tok.inner.description()
                            ))
                        }
                    }
                }

                Err(spanned_error!(
                    brace.span,
                    "unmatched delimeter; expected closing `}}`"
                ))
            }
            Some(tok) => Err(Diagnostic::referencing_error(
                tok.span.clone(),
                format!(
                    "expected argument definition or rule definition, found {}",
                    tok.inner.description()
                ),
                Reference::new(proc.span, "expected as part of this macro"),
            )),
            None => Err(error!(
                "expected argument definition or rule defninition, found `eof`"
            )),
        }
    }
}

pub struct Path {
    pub path: PathInner,
    pub span: Arc<Span>,
}

impl Path {
    fn span(open: &Token![<], close: &Token![>]) -> Arc<Span> {
        Arc::new(Span {
            line: open.span.line,
            range: open.span.range.start..close.span.range.end,
            source: open.span.source.clone(),
        })
    }
}

impl Parsable for Path {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let open: Token![<] = cursor.parse()?;
        let path = cursor.parse()?;
        let close: Token![>] = cursor.parse()?;
        let span = Path::span(&open, &close);
        Ok(Path { path, span })
    }
}

pub enum PathInner {
    Quoted(LitString),
    Unquoted(Punctuated<Ident, Token![/]>),
}

impl Parsable for PathInner {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        if let Some(tok) = cursor.peek() {
            match tok.inner {
                TokenInner::String(_) => return cursor.parse().map(|lit| PathInner::Quoted(lit)),
                _ => {
                    return punctuated!(cursor, TokenInner::Punctuation(Punctuation::Gt))
                        .map(|p| PathInner::Unquoted(p))
                }
            }
        } else {
            Err(error!("expected path, found `eof`"))
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Immediate(Immediate),
    Variable(Variable),
    Label(Ident),
}

impl Parsable for Address {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        match cursor.peek() {
            Some(Token {
                span: _,
                inner: TokenInner::Immediate(_),
            }) => Ok(Address::Immediate(cursor.parse()?)),
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::Variable(_)),
            }) => Ok(Address::Variable(cursor.parse()?)),
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::Ident(_)),
            }) => Ok(Address::Label(cursor.parse()?)),
            Some(_) => {
                let next = unsafe { cursor.next().unwrap_unchecked() };
                Err(spanned_error!(
                    next.span,
                    "expected address, found {}",
                    next.inner.description()
                ))
            }
            None => Err(error!("expected address, found `eof`")),
        }
    }
}

struct VariableDef {
    name: Ident,
    size: u16,
}

impl Parsable for VariableDef {
    fn parse(cursor: &mut Cursor) -> Result<Self, Diagnostic> {
        let size: u16 = match cursor.next() {
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Byte)),
            }) => 1,
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Double)),
            }) => 2,
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Quad)),
            }) => 4,
            Some(Token {
                span: _,
                inner: TokenInner::Ident(lex::Ident::PreProc(PreProc::Var)),
            }) => {
                let size: Immediate = cursor.parse()?;
                size.value.try_into().map_err(|_| {
                    spanned_error!(size.span, "variable size out of range")
                        .with_help("variable size must fit into an unsigned 16-bit integer")
                })?
            }
            Some(tok) => {
                return Err(spanned_error!(
                    tok.span,
                    "expected variable definition, fround {}",
                    tok.inner.description()
                ))
            }
            None => return Err(error!("expected variable definition, found `eof`")),
        };

        let name: Ident = cursor.parse()?;

        let _: NewLine = cursor.parse()?;

        Ok(VariableDef { name, size })
    }
}
