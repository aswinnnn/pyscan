//! Python dependency parser for [PEP 508](https://peps.python.org/pep-0508)
//!
//! ```
//! # use pep_508::*;
//! let dep = "requests[security, socks] <= 2.28.1, == 2.28.*; python_version > '3.7' and extra == 'http'";
//! let parsed = parse(dep).unwrap();
//! let expected = Dependency {
//!     name: "requests",
//!     extras: vec!["security", "socks"],
//!     spec: Some(Spec::Version(vec![
//!         VersionSpec {
//!             comparator: Comparator::Le,
//!             version: "2.28.1",
//!         },
//!         VersionSpec {
//!             comparator: Comparator::Eq,
//!             version: "2.28.*",
//!         },
//!     ])),
//!     marker: Some(Marker::And(
//!         Box::new(Marker::Operator(
//!             Variable::PythonVersion,
//!             Operator::Comparator(Comparator::Gt),
//!             Variable::String("3.7"),
//!         )),
//!         Box::new(Marker::Operator(
//!             Variable::Extra,
//!             Operator::Comparator(Comparator::Eq),
//!             Variable::String("http"),
//!         )),
//!     )),
//! };
//! assert_eq!(parsed, expected);
//! ```
mod macros;
mod url;

use chumsky::{
    error::Error,
    extra::Full,
    prelude::Simple,
    primitive::{any, choice, empty, end, group, just},
    recursive::recursive,
    IterParser, Parser,
};

use macros::set;

/// Python dependency specified by [PEP 508](https://peps.python.org/pep-0508)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dependency<'a> {
    /// Name of the dependency
    pub name: &'a str,
    /// Extras for the dependency, things that go inside `[]`
    pub extras: Vec<&'a str>,
    /// Version specification or URL
    pub spec: Option<Spec<'a>>,
    /// Environment markers, conditions that go after `;`
    pub marker: Option<Marker<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Spec<'a> {
    /// `foo @ https://example.com`
    Url(&'a str),
    /// `foo >= 0.1.0, < 0.2.0`
    Version(Vec<VersionSpec<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionSpec<'a> {
    pub comparator: Comparator,
    pub version: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Marker<'a> {
    And(Box<Marker<'a>>, Box<Marker<'a>>),
    Or(Box<Marker<'a>>, Box<Marker<'a>>),
    Operator(Variable<'a>, Operator, Variable<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Variable<'a> {
    PythonVersion,
    PythonFullVersion,
    OsName,
    SysPlatform,
    PlatformRelease,
    PlatformSystem,
    PlatformVersion,
    PlatformMachine,
    PlatformPythonImplementation,
    ImplementationName,
    ImplementationVersion,
    Extra,
    String(&'a str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    Comparator(Comparator),
    /// `in`
    In,
    /// `not in`
    NotIn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Comparator {
    /// `foo < '0.1.0'`
    Lt,
    /// `foo < '0.1.0'`
    Le,
    /// `foo != '0.1.0'`
    Ne,
    /// `foo == '0.1.0'`
    Eq,
    /// `foo >= '0.1.0'`
    Ge,
    /// `foo > '0.1.0'`
    Gt,
    /// `foo ~= '0.1.0'`
    Cp,
    /// `foo === '0.1.0'`
    Ae,
}

/// Parse a [PEP 508](https://peps.python.org/pep-0508) string into a [Dependency]
/// ```
/// # use pep_508::parse;
/// assert_eq!(parse("requests >= 2").unwrap().name, "requests");
/// assert_eq!(parse("numpy").unwrap().name, "numpy");
/// ```
pub fn parse(dependency: &str) -> Result<Dependency, Vec<Simple<char>>> {
    parser().then_ignore(end()).parse(dependency).into_result()
}

/// Create a [chumsky](https://docs.rs/chumsky) parser,
/// allows more customization than [parse]
pub fn parser<'a, E: Error<'a, &'a str> + 'a>(
) -> impl Parser<'a, &'a str, Dependency<'a>, Full<E, (), ()>> {
    let ws = set!(' ' | '\t').repeated().ignored();
    let ident = any()
        .filter(char::is_ascii_alphanumeric)
        .then(
            set!('-' | '_' | '.')
                .or_not()
                .then(any().filter(char::is_ascii_alphanumeric))
                .repeated(),
        )
        .slice();

    let cmp = choice((
        just("===").to(Comparator::Ae),
        just("<=").to(Comparator::Le),
        just("!=").to(Comparator::Ne),
        just("==").to(Comparator::Eq),
        just(">=").to(Comparator::Ge),
        just("~=").to(Comparator::Cp),
        just('<').to(Comparator::Lt),
        just('>').to(Comparator::Gt),
    ));

    let version_spec = cmp
        .then_ignore(ws)
        .then(
            set!(
                'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '_' | '.' | '*' | '+' | '!'
            )
            .repeated()
            .at_least(1)
            .slice(),
        )
        .map(|(comparator, version)| VersionSpec {
            comparator,
            version,
        })
        .then_ignore(ws)
        .separated_by(just(',').ignore_then(ws))
        .at_least(1)
        .collect();

    group((
        ws.ignore_then(ident).then_ignore(ws),
        ident
            .then_ignore(ws)
            .separated_by(just(',').ignore_then(ws))
            .at_least(1)
            .collect()
            .delimited_by(just('[').ignore_then(ws), just(']'))
            .then_ignore(ws)
            .or(empty().map(|_| Vec::new())),
        just('@')
            .ignore_then(ws)
            .ignore_then(url::parser())
            .map(Spec::Url)
            .or(version_spec
                .delimited_by(just('(').then_ignore(ws), just(')'))
                .or(version_spec)
                .map(Spec::Version))
            .then_ignore(ws)
            .or_not(),
        just(';')
            .ignore_then(ws)
            .ignore_then(recursive(|marker_or| {
                macro_rules! c {
                    () => {
                        ' ' | '\t' | 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '(' | ')' | '.' |
                        '{' | '}' | '-' | '_' | '*' | '#' | ':' | ';' | ',' | '/' | '?' | '[' |
                        ']' | '!' | '~' | '`' | '@' | '$' | '%' | '^' | '&' | '=' | '+' | '|' |
                        '<' | '>'
                    };
                }

                let marker_var = choice((
                    just('\'')
                        .ignore_then(set!(c!() | '"').repeated().slice())
                        .then_ignore(just('\''))
                        .map(Variable::String),
                    just('"')
                        .ignore_then(set!(c!() | '\'').repeated().slice())
                        .then_ignore(just('"'))
                        .map(Variable::String),
                    just("python_version").to(Variable::PythonVersion),
                    just("python_full_version").to(Variable::PythonFullVersion),
                    just("os_name").to(Variable::OsName),
                    just("sys_platform").to(Variable::SysPlatform),
                    just("platform_release").to(Variable::PlatformRelease),
                    just("platform_system").to(Variable::PlatformSystem),
                    just("platform_version").to(Variable::PlatformVersion),
                    just("platform_machine").to(Variable::PlatformMachine),
                    just("platform_python_implementation")
                        .to(Variable::PlatformPythonImplementation),
                    just("implementation_name").to(Variable::ImplementationName),
                    just("implementation_version").to(Variable::ImplementationVersion),
                    just("extra").to(Variable::Extra),
                ));

                let marker_expr = group((
                    marker_var.clone().then_ignore(ws),
                    cmp.map(Operator::Comparator)
                        .or(just("in").to(Operator::In).or(just("not")
                            .ignore_then(set!(' ' | '\t').repeated().at_least(1))
                            .ignore_then(just("in"))
                            .to(Operator::NotIn)))
                        .then_ignore(ws),
                    marker_var,
                ))
                .map(|(lhs, op, rhs)| Marker::Operator(lhs, op, rhs))
                .or(marker_or
                    .then_ignore(ws)
                    .delimited_by(just('(').then_ignore(ws), just(')')));

                let marker_and = marker_expr
                    .clone()
                    .then(
                        ws.ignore_then(just("and"))
                            .ignore_then(ws)
                            .ignore_then(marker_expr)
                            .or_not(),
                    )
                    .map(|(lhs, rhs)| match rhs {
                        Some(rhs) => Marker::And(Box::new(lhs), Box::new(rhs)),
                        None => lhs,
                    });

                marker_and
                    .clone()
                    .then(
                        ws.ignore_then(just("or"))
                            .ignore_then(ws)
                            .ignore_then(marker_and)
                            .or_not(),
                    )
                    .map(|(lhs, rhs)| match rhs {
                        Some(rhs) => Marker::Or(Box::new(lhs), Box::new(rhs)),
                        None => lhs,
                    })
            }))
            .or_not(),
    ))
    .then_ignore(ws)
    .map(|(name, extras, spec, marker)| Dependency {
        name,
        extras,
        spec,
        marker,
    })
}
