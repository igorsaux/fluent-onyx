use fluent::{FluentArgs, FluentValue};

/// Вставляет "The/the" при отсутствии.
pub fn the<'a>(positional: &[FluentValue<'a>], named: &FluentArgs) -> FluentValue<'a> {
    let word = match &positional[0] {
        FluentValue::String(s) => s.to_owned(),
        _ => return FluentValue::Error,
    };

    let words: Vec<_> = word.split_whitespace().collect();

    if words[0].to_lowercase().as_str() == "the" {
        return positional[0].clone();
    };

    if word.to_lowercase().starts_with("the") {
        return FluentValue::String(word);
    }

    let uppercase = match named.get("case") {
        Some(FluentValue::String(s)) => matches!(s.as_ref(), "upper"),
        _ => false,
    };

    let result = format!("{}he {word}", if uppercase { "T" } else { "t" });

    FluentValue::String(result.into())
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

/// Вставляет a/A или an/An при отсутствии.
pub fn a<'a>(positional: &[FluentValue<'a>], named: &FluentArgs) -> FluentValue<'a> {
    let word = match &positional[0] {
        FluentValue::String(s) => s.to_string(),
        _ => return FluentValue::Error,
    };

    let words: Vec<_> = word.split_whitespace().collect();

    match words[0].to_lowercase().as_str() {
        "an" => return positional[0].clone(),
        "a" => return positional[0].clone(),
        _ => (),
    };

    let uppercase = match named.get("case") {
        Some(FluentValue::String(s)) => matches!(s.as_ref(), "upper"),
        _ => false,
    };

    let chars: Vec<_> = word.chars().collect();
    let an = VOWELS.contains(&chars[0]);

    let result = if uppercase {
        format!("A{} {word}", if an { "n" } else { "" })
    } else {
        format!("a{} {word}", if an { "n" } else { "" })
    };

    FluentValue::String(result.into())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fast_bundle, fast_format};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_the() {
        let mut bundle = fast_bundle(
            "en",
            r#"
lowercase = { THE("sun") }
uppercase = { THE("sun", case: "upper") }"#,
        );

        bundle.add_function("THE", super::the).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "the sun");
        assert_eq!(fast_format(&bundle, "uppercase"), "The sun");
    }

    #[test]
    fn test_no_more_the() {
        let mut bundle = fast_bundle(
            "en",
            r#"
lowercase = { THE("the sun") }
uppercase = { THE("The sun", case: "upper") }"#,
        );

        bundle.add_function("THE", super::the).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "the sun");
        assert_eq!(fast_format(&bundle, "uppercase"), "The sun");
    }

    #[test]
    fn test_a() {
        let mut bundle = fast_bundle(
            "en",
            r#"
lowercase = { A("bucket") }
uppercase = { A("bucket", case: "upper") }
"#,
        );

        bundle.add_function("A", super::a).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "a bucket");
        assert_eq!(fast_format(&bundle, "uppercase"), "A bucket");
    }

    #[test]
    fn test_an() {
        let mut bundle = fast_bundle(
            "en",
            r#"
lowercase = { A("apple") }
uppercase = { A("apple", case: "upper") }
"#,
        );

        bundle.add_function("A", super::a).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "an apple");
        assert_eq!(fast_format(&bundle, "uppercase"), "An apple");
    }

    #[test]
    fn test_no_more_a() {
        let mut bundle = fast_bundle(
            "en",
            r#"
lowercase = { A("a bucket") }
uppercase = { A("A bucket", case: "upper") }
"#,
        );

        bundle.add_function("A", super::a).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "a bucket");
        assert_eq!(fast_format(&bundle, "uppercase"), "A bucket");
    }
}
