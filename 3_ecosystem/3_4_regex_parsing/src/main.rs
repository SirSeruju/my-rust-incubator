fn main() {}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

mod custom {
    use super::*;
    use combine::parser::char::{char, digit};
    use combine::stream::position;
    use combine::Parser;
    use combine::{attempt, many1, none_of, one_of, optional, skip_many, EasyParser};

    #[allow(unused)]
    pub fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
        let sign = optional(one_of("+-".chars()).map(|c| match c {
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            _ => unreachable!(),
        }));
        let width = optional(many1(digit()).map(|d: String| d.parse::<usize>().unwrap()));
        let precision = optional(
            (
                char('.'),
                attempt(
                    many1(digit())
                        .skip(char('$'))
                        .map(|d: String| Precision::Argument(d.parse::<usize>().unwrap())),
                )
                .or(many1(digit()).map(|d: String| Precision::Integer(d.parse::<usize>().unwrap())))
                .or(char('*').map(|_| Precision::Asterisk)),
            )
                .map(|t| t.1),
        );
        // TODO: find alternative with something like none_of(digit()) [didn't work cause digit() not an Iterator]
        let prev = skip_many(none_of("+-0123456789".chars()));
        let result = (prev, sign, width, precision)
            .map(|t| (t.1, t.2, t.3))
            .easy_parse(position::Stream::new(input));
        match result {
            Err(_) => (None, None, None),
            Ok(r) => r.0,
        }
    }

    #[cfg(test)]
    mod spec {
        use super::*;

        #[test]
        fn parses_sign() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", None),
                (">+8.*", Some(Sign::Plus)),
                ("-.1$x", Some(Sign::Minus)),
                ("a^#043.8?", None),
            ] {
                let (sign, ..) = parse(input);
                assert_eq!(sign, expected);
            }
        }

        #[test]
        fn parses_width() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", Some(8)),
                (">+8.*", Some(8)),
                ("-.1$x", None),
                ("a^#043.8?", Some(43)),
            ] {
                let (_, width, _) = parse(input);
                assert_eq!(width, expected);
            }
        }

        #[test]
        fn parses_precision() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", Some(Precision::Asterisk)),
                (">+8.*", Some(Precision::Asterisk)),
                ("-.1$x", Some(Precision::Argument(1))),
                ("a^#043.8?", Some(Precision::Integer(8))),
            ] {
                let (_, _, precision) = parse(input);
                assert_eq!(precision, expected);
            }
        }
    }
}

mod regexp {
    use super::*;
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::unreachable;

    #[allow(unused)]
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[^\d+-]*(?<sign>(\+|\-))?((?<width>\d*))?(\.(?<precision>(\*|\d+\$|\d*)))?")
            .unwrap()
    });
    #[allow(unused)]
    pub fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
        let caps = REGEX.captures(input).unwrap();
        let sign = caps.name("sign").map(|s| match s.as_str() {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => unreachable!(),
        });
        let width = match caps.name("width") {
            None => None,
            Some(w) => w.as_str().parse::<usize>().ok(),
        };
        let precision = caps.name("precision").map(|p| match p.as_str() {
            "*" => Some(Precision::Asterisk),
            a if a.ends_with('$') => a[..a.len() - 1]
                .parse::<usize>()
                .ok()
                .map(Precision::Argument),
            i => i.parse::<usize>().ok().map(Precision::Integer),
        });
        let precision = match precision {
            None => None,
            Some(p) => p,
        };
        (sign, width, precision)
    }
    #[cfg(test)]
    mod spec {
        use super::*;

        #[test]
        fn parses_sign() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", None),
                (">+8.*", Some(Sign::Plus)),
                ("-.1$x", Some(Sign::Minus)),
                ("a^#043.8?", None),
            ] {
                let (sign, ..) = parse(input);
                assert_eq!(sign, expected);
            }
        }

        #[test]
        fn parses_width() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", Some(8)),
                (">+8.*", Some(8)),
                ("-.1$x", None),
                ("a^#043.8?", Some(43)),
            ] {
                let (_, width, _) = parse(input);
                assert_eq!(width, expected);
            }
        }

        #[test]
        fn parses_precision() {
            for (input, expected) in vec![
                ("", None),
                (">8.*", Some(Precision::Asterisk)),
                (">+8.*", Some(Precision::Asterisk)),
                ("-.1$x", Some(Precision::Argument(1))),
                ("a^#043.8?", Some(Precision::Integer(8))),
            ] {
                let (_, _, precision) = parse(input);
                assert_eq!(precision, expected);
            }
        }
    }
}
