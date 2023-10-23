use chumsky::{
    error::Error,
    extra::Full,
    primitive::{any, choice, group, just},
    Parser,
};

use super::macros::set;

macro_rules! c {
    () => {
        'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '.' | '_' | '~' |
        '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
    };
}

pub(crate) fn parser<'a, E: Error<'a, &'a str> + 'a>(
) -> impl Parser<'a, &'a str, &'a str, Full<E, (), ()>> {
    let c = set!(c!());
    let digit = any().filter(char::is_ascii_digit);
    let hex = any().filter(char::is_ascii_hexdigit);
    let percent = just('%').then_ignore(hex.repeated().exactly(2).rewind());
    let reg = percent.or(c);
    let pchar = percent.or(set!(c!() | ':' | '@'));

    let octet = choice((
        group((just('1'), digit, digit)).ignored(),
        just('2')
            .then(
                set!('0' ..= '4')
                    .then(digit)
                    .or(just('5').then(set!('0' ..= '5'))),
            )
            .ignored(),
        set!('1' ..= '9').then(digit).ignored(),
        digit.map(|c| vec![c]).ignored(),
    ));
    let ipv4 = group((octet, just('.'), octet, just('.'), octet, just('.'), octet)).ignored();
    let h16 = hex.repeated().at_least(1).at_most(4);
    let h16r = just(':').then(h16).repeated();
    let ls32 = group((h16, just(':'), h16)).ignored().or(ipv4);

    let segments = just('/').then(pchar.repeated()).repeated();
    let frag = percent.or(set!(c!() | ':' | '@' | '/' | '?')).repeated();
    let frags = just('?')
        .then(frag)
        .or_not()
        .then(just('#').then(frag).or_not());

    let path = just('/')
        .then(
            group((
                just('/'),
                percent
                    .or(set!(c!() | ':'))
                    .repeated()
                    .then(just('@'))
                    .or_not(),
                choice((
                    just('[')
                        .then(choice((
                            group((
                                just('v'),
                                hex.repeated().at_least(1),
                                just('.'),
                                set!(c!() | ':').repeated().at_least(1),
                            ))
                            .ignored(),
                            h16.then(just(':'))
                                .repeated()
                                .exactly(6)
                                .then(ls32)
                                .ignored(),
                            group((just("::"), h16r.exactly(5), ls32)).ignored(),
                            group((h16.or_not(), just("::"), h16r.exactly(4), ls32)).ignored(),
                            group((
                                h16.then(just(':')).or_not().then(h16).or_not(),
                                just("::"),
                                h16r.exactly(3),
                                ls32,
                            ))
                            .ignored(),
                            group((
                                h16.then(h16r.at_most(2)).or_not(),
                                just(':'),
                                just(':'),
                                h16r.exactly(2),
                                ls32,
                            ))
                            .ignored(),
                            group((
                                h16.then(h16r.at_most(3)).or_not(),
                                just("::"),
                                h16,
                                just(':'),
                                ls32,
                            ))
                            .ignored(),
                            group((h16.then(h16r.at_most(4)).or_not(), just("::"), ls32)).ignored(),
                            group((h16.then(h16r.at_most(5)).or_not(), just("::"), h16)).ignored(),
                            h16.then(h16r.at_most(6))
                                .or_not()
                                .then(just("::"))
                                .ignored(),
                        )))
                        .then(just(']'))
                        .ignored(),
                    ipv4,
                    reg.repeated(),
                )),
                just(':').then(digit.repeated()).or_not(),
                segments,
            ))
            .ignored()
            .or(pchar
                .repeated()
                .at_least(1)
                .then(segments)
                .or_not()
                .ignored())
            .or_not(),
        )
        .ignored();

    choice((
        group((
            any().filter(char::is_ascii_alphabetic),
            set!('A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '+' | '-' | '.').repeated(),
            just(':'),
            path.or(pchar.repeated().at_least(1).then(segments).ignored()),
        ))
        .ignored(),
        path,
        percent
            .or(set!(c!() | '@'))
            .repeated()
            .at_least(1)
            .then(segments)
            .ignored(),
    ))
    .then(frags)
    .slice()
}

#[cfg(test)]
mod tests {
    use chumsky::{prelude::Rich, primitive::end, Parser};

    use super::parser;

    fn parse(s: &str) -> Result<&str, Vec<Rich<char>>> {
        parser().then_ignore(end()).parse(s).into_result()
    }

    fn check(urls: impl IntoIterator<Item = impl AsRef<str>>) {
        for url in urls {
            let url = url.as_ref();
            assert_eq!(url, parse(url).expect(url));
        }
    }

    #[test]
    fn basic() {
        check([
            "https://github.com/figsoda/pep-508",
            "https://crates.io/search?q=pep-508&sort=recent-downloads",
            "http://127.0.0.1:8000?some=query#anchor",
            "/relative/url?query=good",
            "another/relative/url#this",
        ]);

        assert!(parse("").is_err());
        assert!(parse("https://example.com ").is_err());
    }

    // examples from https://datatracker.ietf.org/doc/html/rfc3986.htm
    #[test]
    fn examples() {
        check([
            "ftp://ftp.is.co.za/rfc/rfc1808.txt",
            "http://www.ietf.org/rfc/rfc2396.txt",
            "ldap://[2001:db8::7]/c=GB?objectClass?one",
            "mailto:John.Doe@example.com",
            "news:comp.infosystems.www.servers.unix",
            "tel:+1-816-555-1212",
            "telnet://192.0.2.16:80/",
            "urn:oasis:names:specification:docbook:dtd:xml:4.1.2",
        ]);
    }

    #[test]
    fn ipv6() {
        check([
            "https://[::]",
            "https://[::1]",
            "https://[0:0:0:0:0:0:0:0]",
            "https://[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]",
        ]);

        assert!(parse("[::]").is_err());
        assert!(parse("https://[:::1]").is_err());
        assert!(parse("https://[ffff:ffff:ffff:ffff:ffff:ffff:ffff]").is_err());
        assert!(parse("https://[0:0:0:0:0:0:0:0:0]").is_err());
    }
}
