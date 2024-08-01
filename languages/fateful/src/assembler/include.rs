use std::{
    collections::HashMap,
    fs,
    path::{PathBuf, MAIN_SEPARATOR_STR},
    sync::Arc,
};

use super::{
    lex::{self, Span, TokenStream},
    parse::{Path, PathInner},
    Diagnostic, Errors,
};
use crate::{note, spanned_error};

use clio::Input;
use git2::Repository;

const CACHE_DIR: &str = ".fateful-cache";

#[derive(Debug)]
pub struct Lib {
    pub name_span: Arc<Span>,
    source_span: Arc<Span>,
    source: LibSource,
}

impl Lib {
    pub fn new(source: String, name_span: Arc<Span>, source_span: Arc<Span>) -> Lib {
        Lib {
            name_span,
            source_span,
            source: LibSource::new(source),
        }
    }

    fn make_local(&mut self, name: &str) -> Result<(), Diagnostic> {
        match self.source {
            LibSource::Local(_) => {}
            LibSource::Net {
                ref url,
                ref mut path,
            } => {
                if path.is_none() {
                    let download_path = CACHE_DIR.to_owned() + MAIN_SEPARATOR_STR + name;
                    // create lib cache here if it does not exist so that no cache is created if no libraries are downloaded
                    fs::create_dir_all(CACHE_DIR).map_err(|err| {
                        spanned_error!(
                            self.source_span.clone(),
                            "failed to create library cache: {}",
                            err
                        )
                    })?;

                    if std::path::Path::new(&download_path).is_dir() {
                        fs::remove_dir_all(&download_path).map_err(|err| {
                            spanned_error!(
                                self.name_span.clone(),
                                "unable to remove preexisting directory: {err}"
                            )
                        })?;
                    }

                    Repository::clone(url, &download_path).map_err(|err| {
                        spanned_error!(
                            self.source_span.clone(),
                            "unable to clone repository: {}",
                            err.message()
                        )
                    })?;

                    *path = Some(download_path);
                }
            }
        }

        Ok(())
    }
}

impl PartialEq<&str> for Lib {
    fn eq(&self, other: &&str) -> bool {
        match self.source {
            LibSource::Local(ref path) => path == other,
            LibSource::Net { ref url, path: _ } => url == other,
        }
    }
}

#[derive(Debug)]
pub enum LibSource {
    Net { url: String, path: Option<String> },
    Local(String),
}

impl LibSource {
    pub fn new(source: String) -> LibSource {
        if source.starts_with("https://") || source.starts_with("http://") {
            LibSource::Net {
                url: source,
                path: None,
            }
        } else {
            LibSource::Local(source)
        }
    }
}

pub fn include_builtins() -> Result<TokenStream, Errors> {
    lex::lex_string(Some("builtin macros"), include_str!("macros.asm"))
}

pub fn include(path: Path, libs: &mut HashMap<String, Lib>) -> Result<TokenStream, Errors> {
    match path.path {
        PathInner::Quoted(s) => lex::lex(
            Input::new(&s.value.to_string())
                .map_err(|err| vec![spanned_error!(s.span, "unable to read input; {err}")])?,
        ),
        PathInner::Unquoted(p) => {
            let err_span = path.span.clone();
            let locator = p
                .first()
                .ok_or_else(|| vec![spanned_error!(err_span, "expected library name")])?;

            let err_span = path.span.clone();
            let lib = libs
                .get_mut(&locator.value)
                .ok_or_else(|| vec![spanned_error!(err_span, "library not imported")])?;
            lib.make_local(&locator.value).map_err(|err| vec![err])?;

            let path = match &lib.source {
                LibSource::Local(lib_path) => PathBuf::from(lib_path).join(PathBuf::from_iter(
                    p.values().skip(1).map(|ident| &ident.value),
                )),
                LibSource::Net {
                    url: _,
                    path: Some(path),
                } => PathBuf::from(path).join(PathBuf::from_iter(
                    p.values().skip(1).map(|ident| &ident.value),
                )),
                _ => unreachable!(),
            };

            note!("reading imported file: {}", path.display()).emit();

            lex::lex(Input::new(&path).map_err(|err| {
                vec![spanned_error!(
                    p.last().unwrap().span.clone(),
                    "unable to read file `{path}`: {err}",
                    path = path.display()
                )]
            })?)
        }
    }
}
