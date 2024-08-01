use std::fmt;
use std::io::{BufRead, ErrorKind};
use std::ops::Range;
use std::str::FromStr;
use std::sync::Arc;

use super::ascii::{unescape_str, AsciiStr, UnescapeError};
use super::Errors;
use crate::{diagnostic::Diagnostic, error};
use clio::{ClioPath, Input};
use logos::{Lexer, Logos};

pub type TokenStream = Vec<Token>;
pub type LexResult = std::result::Result<TokenStream, Errors>;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub inner: TokenInner,
    pub span: Arc<Span>,
}

impl TokenInner {
    pub const fn description(&self) -> &'static str {
        use TokenInner as TI;
        match self {
            TI::Delimeter(delim) => delim.description(),
            TI::Doc(_) => "doc string",
            TI::Ident(ref ident) => ident.description(),
            TI::Immediate(_) => "immediate",
            TI::String(_) => "string",
            TI::NewLine => "newline",
            TI::Punctuation(punct) => punct.description(),
            TI::Location => "location",
        }
    }
}

impl FromStr for Token {
    type Err = Diagnostic;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let skipped = s.replace("\\\n", "");
        let mut lex = TokenInner::lexer(&skipped).spanned();
        let (token, span) = lex
            .next()
            .ok_or_else(|| error!("No tokens found in string"))?;
        let span = Arc::new(Span {
            line: 0,
            range: span,
            source: Source::String {
                source: Arc::new(s.to_owned()),
                name: None,
            },
        });
        match token {
            Ok(inner) => Ok(Token { inner, span }),
            Err(mut err) => {
                err.set_span(span);
                Err(err)
            }
        }
    }
}

pub fn lex(mut input: Input) -> LexResult {
    let mut tokens: TokenStream = Vec::new();
    let mut errs: Errors = Vec::new();
    let source = Arc::new(input.path().to_owned());

    let mut prev_lines = String::new();
    let mut multiline = false;

    let reader = input.lock();
    for (line_num, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                // Treat multiple lines with `\` between them as one line
                if !multiline {
                    prev_lines.clear();
                }
                if line.ends_with('\\') {
                    prev_lines += &line[..line.len() - 1];
                    multiline = true;
                    continue;
                } else {
                    multiline = false;
                }

                for (token, span) in TokenInner::lexer(&(prev_lines.clone() + &line)).spanned() {
                    let spanned = span.start + prev_lines.len()..span.end + prev_lines.len();
                    let span = Arc::new(Span {
                        line: line_num,
                        range: spanned,
                        source: Source::File(source.clone()),
                    });
                    match token {
                        Ok(tok) => tokens.push(Token { inner: tok, span }),
                        Err(mut err) => {
                            err.set_span(span);
                            errs.push(err);
                        }
                    }
                }

                tokens.push(Token {
                    inner: TokenInner::NewLine,
                    span: Arc::new(Span {
                        line: line_num,
                        range: line.len()..(line.len() + 1),
                        source: Source::File(source.clone()),
                    }),
                })
            }
            Err(err) => {
                errs.push(Diagnostic::error(match err.kind() {
                    ErrorKind::InvalidData => format!("encountered invalid data on line {line_num} (likely not valid UTF-8)"),
                    _ => format!("encountered an unexpected error while reading the input file on line {line_num}: {}", err.kind()),
                }));
                break;
            }
        }
    }

    if errs.is_empty() {
        Ok(tokens)
    } else {
        Err(errs)
    }
}

pub fn lex_string<S>(name: Option<&'static str>, file: S) -> LexResult
where
    S: Into<String>,
{
    let mut tokens: TokenStream = Vec::new();
    let mut errs: Errors = Vec::new();
    let source = Arc::new(file.into());

    let mut prev_lines = String::new();
    let mut multiline = false;

    for (line_num, line) in source.lines().enumerate() {
        // Treat multiple lines with `\` between them as one line
        if !multiline {
            prev_lines.clear();
        }
        if line.ends_with('\\') {
            prev_lines += &line[..line.len() - 1];
            multiline = true;
            continue;
        } else {
            multiline = false;
        }

        for (token, span) in TokenInner::lexer(&(prev_lines.clone() + &line)).spanned() {
            let spanned = span.start + prev_lines.len()..span.end + prev_lines.len();
            let span = Arc::new(Span {
                line: line_num,
                range: spanned,
                source: Source::String {
                    name,
                    source: source.clone(),
                },
            });
            match token {
                Ok(tok) => tokens.push(Token { inner: tok, span }),
                Err(mut err) => {
                    err.set_span(span);
                    errs.push(err);
                }
            }
        }

        tokens.push(Token {
            inner: TokenInner::NewLine,
            span: Arc::new(Span {
                line: line_num,
                range: line.len()..(line.len() + 1),
                source: Source::String {
                    name,
                    source: source.clone(),
                },
            }),
        })
    }

    if errs.is_empty() {
        Ok(tokens)
    } else {
        Err(errs)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error = Diagnostic)]
#[logos(skip r"(//|;|#)[^\n]*\n?")]
#[logos(skip r"/\*(?:[^*]|\*[^/])*\*/")]
#[logos(skip r"[ \t\f]")]
pub enum TokenInner {
    #[regex(r"0b[01][_01]*", TokenInner::binary)]
    #[regex(r"0o[0-7][_0-7]*", TokenInner::octal)]
    #[regex(r"-?[0-9][_0-9]*", TokenInner::decimal)]
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", TokenInner::hexadecimal)]
    #[regex(r"'[\x00-\x7F]*'", TokenInner::char)]
    #[regex(r#"'\\[(\\)n"at0rbfv]'"#, TokenInner::char)]
    #[regex(r"'\\x[[:xdigit:]]{1,2}'", TokenInner::char)]
    Immediate(i128),

    #[regex(r#""((\\")|[\x00-\x21\x23-\x7F])*""#, TokenInner::string)]
    #[regex(r##"r#"((\\")|[\x00-\x21\x23-\x7F])*"#"##, TokenInner::raw_string)]
    String(AsciiStr),

    #[regex(r"[._a-zA-Z][._a-zA-Z0-9]*", Ident::any)]
    #[regex(r"@[_a-zA-Z][_a-zA-Z0-9]*", Ident::pre_proc)]
    #[regex(r"%[_a-zA-Z][_a-zA-Z0-9]*", Ident::macro_variable)]
    #[regex(r"\$[_a-zA-Z][_a-zA-Z0-9]*", Ident::variable)]
    Ident(Ident),

    #[token("(", Delimeter::open_paren)]
    #[token(")", Delimeter::close_paren)]
    #[token("[", Delimeter::open_bracket)]
    #[token("]", Delimeter::close_bracket)]
    #[token("{", Delimeter::open_brace)]
    #[token("}", Delimeter::close_brace)]
    Delimeter(Delimeter),

    #[token("=", Punctuation::eq)]
    #[token("==", Punctuation::eq_eq)]
    #[token("!=", Punctuation::ne)]
    #[token("<", Punctuation::lt)]
    #[token("<=", Punctuation::le)]
    #[token(">", Punctuation::gt)]
    #[token(">=", Punctuation::ge)]
    #[token("&", Punctuation::and)]
    #[token("&&", Punctuation::and_and)]
    #[token("|", Punctuation::or)]
    #[token("||", Punctuation::or_or)]
    #[token("^", Punctuation::caret)]
    #[token("!", Punctuation::not)]
    #[token("~", Punctuation::not)]
    #[token("+", Punctuation::plus)]
    #[token("-", Punctuation::minus)]
    #[token("*", Punctuation::star)]
    #[token("/", Punctuation::slash)]
    #[token("<<", Punctuation::shl)]
    #[token(">>", Punctuation::shr)]
    #[token(",", Punctuation::comma)]
    #[token(":", Punctuation::colon)]
    Punctuation(Punctuation),

    #[regex(r"///[^\n]*", TokenInner::doc)]
    #[regex(r"//\*(?:[^*]|\*[^/]|\*/[^/])*\*//", TokenInner::inline_doc)]
    Doc(String),

    #[token("$")]
    Location,

    #[token("\n")]
    NewLine,
}

macro_rules! varient {
    ($($fn:ident -> $ty:ident::$varient:ident),* $(,)?) => {
        $(fn $fn(_: &mut Lexer<TokenInner>) -> $ty {
            $ty::$varient
        })*
    };
}

impl TokenInner {
    fn binary(lex: &mut Lexer<TokenInner>) -> Option<i128> {
        let slice = lex.slice().replace("_", "");
        i128::from_str_radix(&slice.strip_prefix("0b")?, 2).ok()
    }

    fn octal(lex: &mut Lexer<TokenInner>) -> Option<i128> {
        let slice = lex.slice().replace("_", "");
        i128::from_str_radix(&slice.strip_prefix("0o")?, 8).ok()
    }

    fn decimal(lex: &mut Lexer<TokenInner>) -> Option<i128> {
        let slice = lex.slice().replace("_", "");
        i128::from_str_radix(&slice, 10).ok()
    }

    fn hexadecimal(lex: &mut Lexer<TokenInner>) -> Option<i128> {
        let slice = lex.slice().replace("_", "");
        i128::from_str_radix(&slice.strip_prefix("0x")?, 16).ok()
    }

    fn string(lex: &mut Lexer<TokenInner>) -> Result<AsciiStr, Diagnostic> {
        let slice = lex
            .slice()
            .strip_prefix("\"")
            .ok_or_else(|| error!("string not prefixed with `\"`"))?
            .strip_suffix("\"")
            .ok_or_else(|| error!("string not suffixed with `\"`"))?;

        Ok(unescape_str(&slice).map_err(|err| {
            Diagnostic::error(match err {
                UnescapeError::InvalidAscii(byte) => format!("invalid ASCII character: {byte}"),
                UnescapeError::UnmatchedBackslash(index) => {
                    format!("unmatched '\\' at string index {index}")
                }
            })
        })?)
    }

    fn raw_string(lex: &mut Lexer<TokenInner>) -> Result<AsciiStr, Diagnostic> {
        let slice = lex
            .slice()
            .strip_prefix("r#\"")
            .ok_or_else(|| error!("string not prefixed with `r#\"`"))?
            .strip_suffix("#\"")
            .ok_or_else(|| error!("string not suffixed with `\"#`"))?;

        Ok(unescape_str(&slice).map_err(|err| {
            Diagnostic::error(match err {
                UnescapeError::InvalidAscii(byte) => format!("invalid ASCII character: {byte}"),
                UnescapeError::UnmatchedBackslash(index) => {
                    format!("unmatched `\\` at string index {index}")
                }
            })
        })?)
    }

    fn char(lex: &mut Lexer<TokenInner>) -> Result<i128, Diagnostic> {
        let slice = lex.slice();
        Self::char_from_str(slice).map(|c| c.into())
    }

    fn char_from_str(s: &str) -> Result<u8, Diagnostic> {
        let inner = s
            .strip_prefix('\'')
            .ok_or_else(|| error!("char not prefixed with `'`"))?
            .strip_suffix('\'')
            .ok_or_else(|| error!("char not suffixed with `'`"))?;

        let escaped = unescape_str(inner).map_err(|err| {
            Diagnostic::error(match err {
                UnescapeError::InvalidAscii(byte) => format!("invalid ASCII character: {byte}"),
                UnescapeError::UnmatchedBackslash(index) => {
                    format!("unmatched `\\` at string index {index}")
                }
            })
        })?;
        Ok(escaped[0])
    }

    fn doc(lex: &mut Lexer<TokenInner>) -> Result<String, Diagnostic> {
        Ok(lex
            .slice()
            .strip_prefix("///")
            .ok_or_else(|| error!("doc comment does not start with `///`"))?
            .to_owned())
    }

    fn inline_doc(lex: &mut Lexer<TokenInner>) -> Result<String, Diagnostic> {
        Ok(lex
            .slice()
            .strip_prefix("//*")
            .ok_or_else(|| error!("inline doc comment does not start with `//*`").as_bug())?
            .strip_suffix("*//")
            .ok_or_else(|| error!("inline doc comment does not end with `*//`").as_bug())?
            .to_owned())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ident {
    Register(Register),
    PreProc(PreProc),
    Variable(String),
    MacroVariable(String),
    Ty(Ty),
    Ident(String),
}

impl Ident {
    const fn description(&self) -> &'static str {
        match self {
            Ident::Register(_) => "register",
            Ident::PreProc(pp) => pp.description(),
            Ident::Variable(_) => "variable",
            Ident::MacroVariable(_) => "macro variable",
            Ident::Ty(_) => "type",
            Ident::Ident(_) => "identifier",
        }
    }

    fn variable(lex: &mut Lexer<TokenInner>) -> Result<Ident, Diagnostic> {
        let slice = lex
            .slice()
            .strip_prefix("$")
            .ok_or_else(|| error!("variable not prefixed by `$`"))?;
        Ok(Ident::Variable(slice.to_owned()))
    }

    fn macro_variable(lex: &mut Lexer<TokenInner>) -> Result<Ident, Diagnostic> {
        let slice = lex
            .slice()
            .strip_prefix("%")
            .ok_or_else(|| error!("macro variable not prefixed by `%`"))?;
        Ok(Ident::MacroVariable(slice.to_owned()))
    }

    fn pre_proc(lex: &mut Lexer<TokenInner>) -> Result<Ident, Diagnostic> {
        Ok(Ident::PreProc(PreProc::from_str(lex.slice())?))
    }

    fn any(lex: &mut Lexer<TokenInner>) -> Ident {
        let slice = lex.slice().trim();

        if let Ok(register) = Register::from_str(slice) {
            Ident::Register(register)
        } else if let Ok(ty) = Ty::from_str(slice) {
            Ident::Ty(ty)
        } else {
            Ident::Ident(slice.to_owned())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    /// GP register A.
    A = 0,
    /// GP register B.
    B = 1,
    /// GP register C.
    C = 2,
    /// GP register D.
    D = 3,
    /// GP register E.
    E = 4,
    /// Status register
    F = 5,
    /// Memory index low.
    L = 6,
    /// Memory index high.
    H = 7,
}

impl FromStr for Register {
    type Err = Diagnostic;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r0" | "A" => Ok(Register::A),
            "r1" | "B" => Ok(Register::B),
            "r2" | "C" => Ok(Register::C),
            "r3" | "D" => Ok(Register::D),
            "r4" | "E" => Ok(Register::E),
            "r5" | "F" => Ok(Register::F),
            "r6" | "H" => Ok(Register::L),
            "r7" | "L" => Ok(Register::H),
            _ => Err(error!("unknown register")),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PreProc {
    Include,
    Macro,
    Define,
    UnDef,
    IfDef,
    IfNDef,
    If,
    Else,
    ElIf,
    EndIf,
    Org,
    Cseg,
    Dseg,
    Byte,
    Double,
    Quad,
    Str,
    Var,
    Error,
}

impl PreProc {
    const fn description(&self) -> &'static str {
        use PreProc as PP;
        match self {
            PP::Include => "`@include`",
            PP::Macro => "`@macro`",
            PP::Define => "`@define`",
            PP::UnDef => "`@undef`",
            PP::IfDef => "`@ifdef`",
            PP::IfNDef => "`@ifndef`",
            PP::If => "`@if`",
            PP::Else => "`@else`",
            PP::ElIf => "`@elif`",
            PP::EndIf => "`@endif`",
            PP::Org => "`@org`",
            PP::Cseg => "`@cseg`",
            PP::Dseg => "`@dseg`",
            PP::Byte => "`@byte`",
            PP::Double => "`@double`",
            PP::Quad => "`@quad`",
            PP::Str => "`@str`",
            PP::Var => "`@var`",
            PP::Error => "`@error`",
        }
    }
}

impl FromStr for PreProc {
    type Err = Diagnostic;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let argument = s
            .strip_prefix("@")
            .ok_or_else(|| error!("Preprocessor argument not prefixed by `@`"))?;

        use PreProc as PP;

        match argument {
            "include" => Ok(PP::Include),
            "macro" => Ok(PP::Macro),
            "define" => Ok(PP::Define),
            "undef" => Ok(PP::UnDef),
            "ifdef" => Ok(PP::IfDef),
            "ifndef" => Ok(PP::IfNDef),
            "if" => Ok(PP::If),
            "else" => Ok(PP::Else),
            "elif" => Ok(PP::ElIf),
            "endif" => Ok(PP::EndIf),
            "org" => Ok(PP::Org),
            "cseg" => Ok(PP::Cseg),
            "dseg" => Ok(PP::Dseg),
            "byte" => Ok(PP::Byte),
            "double" => Ok(PP::Double),
            "quad" => Ok(PP::Quad),
            "str" => Ok(PP::Str),
            "var" => Ok(PP::Var),
            "error" => Ok(PP::Error),
            _ => Err(error!("Unrecognized preprocessor argument")),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ty {
    Any,
    Reg,
    Addr,
    Label,
    Imm,
    Ident,
    Str,
}

impl FromStr for Ty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "any" => Ok(Ty::Any),
            "reg" => Ok(Ty::Reg),
            "addr" => Ok(Ty::Addr),
            "label" => Ok(Ty::Label),
            "imm" => Ok(Ty::Imm),
            "ident" => Ok(Ty::Ident),
            "str" => Ok(Ty::Str),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Delimeter {
    OpenParen,
    ClosedParen,
    OpenBracket,
    ClosedBracket,
    OpenBrace,
    ClosedBrace,
}

impl Delimeter {
    varient! {
        open_paren -> Delimeter::OpenParen,
        close_paren -> Delimeter::ClosedParen,
        open_bracket -> Delimeter::OpenBracket,
        close_bracket -> Delimeter::ClosedBracket,
        open_brace -> Delimeter::OpenBrace,
        close_brace -> Delimeter::ClosedBrace,
    }

    const fn description(&self) -> &'static str {
        use Delimeter as D;
        match self {
            D::OpenParen => "`(`",
            D::ClosedParen => "`)`",
            D::OpenBracket => "`[`",
            D::ClosedBracket => "`]`",
            D::OpenBrace => "`{`",
            D::ClosedBrace => "`}`",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuation {
    /// `=` (variable assignment)
    Eq,
    /// `=` (*pre-proc eval*: equality)
    EqEq,
    /// `!=` (*pre-proc eval*: not equal)
    Ne,
    /// `<` (*pre-proc eval*: less than)
    Lt,
    /// `<=` (*pre-proc eval*: less or equal)
    Le,
    /// `>` (*pre-proc eval*: greater than)
    Gt,
    /// `>=` (*pre-proc eval*: greater or equal)
    Ge,
    /// `&` (*expression parsing*: bitwise and)
    And,
    /// `&&` (*pre-proc eval*: and)
    AndAnd,
    /// `|` (*expression parsing*: bitwise or, *macro parsing*: type seperator)
    Or,
    /// `||` (*pre-proc eval*: or)
    OrOr,
    /// `^` (*expression parsing*: bitwise xor)
    Caret,
    /// `!` or `~` (*expression parsing*: bitwise not)
    Not,
    /// `/` (*expression parsing*: division) (*pre-proc eval*: path seperator)
    Slash,
    /// `+` (*expression parsing*: addition)
    Plus,
    /// `-` (*expression parsing*: subtraction)
    Minus,
    /// `*` (*expression parsing*: multiplication)
    Star,
    /// `<<` (*expression parsing*: shift left)
    Shl,
    /// `>>` (*expression parsing*: shift right)
    Shr,
    /// `,` (argument seperator)
    Comma,
    /// `:` (label definition, type seperator)
    Colon,
}

impl Punctuation {
    varient! {
        eq -> Punctuation::Eq,
        eq_eq -> Punctuation::EqEq,
        ne -> Punctuation::Ne,
        lt -> Punctuation::Lt,
        le -> Punctuation::Le,
        gt -> Punctuation::Gt,
        ge -> Punctuation::Ge,
        and -> Punctuation::And,
        and_and -> Punctuation::AndAnd,
        or -> Punctuation::Or,
        or_or -> Punctuation::OrOr,
        caret -> Punctuation::Caret,
        not -> Punctuation::Not,
        slash -> Punctuation::Slash,
        plus -> Punctuation::Plus,
        minus -> Punctuation::Minus,
        star -> Punctuation::Star,
        shl -> Punctuation::Shl,
        shr -> Punctuation::Shr,
        comma -> Punctuation::Comma,
        colon -> Punctuation::Colon,
    }

    const fn description(&self) -> &'static str {
        use Punctuation as P;
        match self {
            P::And => "`&`",
            P::AndAnd => "`&&`",
            P::Caret => "`^`",
            P::Colon => "`:`",
            P::Comma => "`,`",
            P::Eq => "`=`",
            P::EqEq => "`==`",
            P::Ge => "`>=`",
            P::Gt => "`>`",
            P::Le => "`<=`",
            P::Lt => "`<`",
            P::Minus => "`-`",
            P::Ne => "`!=`",
            P::Not => "`!` or `~`",
            P::Or => "`|`",
            P::OrOr => "`||`",
            P::Plus => "`+`",
            P::Shl => "`<<`",
            P::Shr => "`>>`",
            P::Slash => "/",
            P::Star => "*",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    File(Arc<ClioPath>),
    String {
        name: Option<&'static str>,
        source: Arc<String>,
    },
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::File(input) => write!(f, "{}", input.display()),
            Source::String { name, source: _ } => match name {
                Some(n) => write!(f, "{n}"),
                None => Ok(()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub range: Range<usize>,
    pub source: Source,
}

impl Span {
    /// Line number for display.
    /// Starts at 1.
    pub fn line_number(&self) -> usize {
        self.line + 1
    }

    pub fn start(&self) -> usize {
        self.range.start
    }

    pub fn end(&self) -> usize {
        self.range.end
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn line(&self) -> Result<String, Diagnostic> {
        match &self.source {
            Source::File(path) => {
                let mut input = (**path)
                    .to_owned()
                    .open()
                    .map_err(|_| error!("Unable to read input {}", path.display()))?;
                let reader = input.lock();

                let line = reader
                    .lines()
                    .nth(self.line)
                    .ok_or_else(|| {
                        error!("Line should be fully contained in the source file").as_bug()
                    })?
                    .map_err(|_| {
                        error!(
                            "Unable to read line {} from file {}",
                            self.line,
                            path.display()
                        )
                    });

                line
            }
            Source::String { name: _, source } => source
                .lines()
                .nth(self.line)
                .ok_or_else(|| {
                    error!("Line should be fully contained in the source string").as_bug()
                })
                .map(|line| line.to_owned()),
        }
    }

    pub fn same_line(start: &Span, end: &Span) -> Arc<Span> {
        Arc::new(Span {
            line: start.line,
            range: start.range.start..end.range.end,
            source: start.source().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delim() {
        let example = "< )".to_owned();

        let lexed = match lex_string(Some("test"), example) {
            Ok(tokens) => tokens,
            Err(errors) => {
                for error in errors {
                    error.force_emit();
                }
                error!("lexing failed due to previous errors").scream();
            }
        };

        println!(
            "{:?}",
            lexed
                .into_iter()
                .map(|tok| tok.inner)
                .collect::<Vec<TokenInner>>()
        );
    }
}
