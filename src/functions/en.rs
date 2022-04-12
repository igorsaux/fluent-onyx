use fluent::{FluentArgs, FluentValue};

/// Вставляет "The/the" при отсутствии.
pub fn the<'a>(positional: &[FluentValue<'a>], named: &FluentArgs) -> FluentValue<'a> {
    let main_word = match &positional[0] {
        FluentValue::String(s) => s.to_owned(),
        _ => return FluentValue::Error,
    };

    if main_word.to_lowercase().starts_with("the") {
        return FluentValue::String(main_word);
    }

    let uppercase = match named.get("case") {
        Some(FluentValue::String(s)) => matches!(s.as_ref(), "upper"),
        _ => false,
    };

    if uppercase {
        FluentValue::String(format!("The {main_word}").into())
    } else {
        FluentValue::String(format!("the {main_word}").into())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fast_bundle, fast_format};

    #[test]
    fn test_the() {
        let mut bundle = fast_bundle(
            "ru",
            r#"
lowercase = { THE("sun") }
uppercase = { THE("sun", case: "upper") }"#,
        );

        bundle.add_function("THE", super::the).unwrap();

        assert_eq!(fast_format(&bundle, "lowercase"), "the sun");
        assert_eq!(fast_format(&bundle, "uppercase"), "The sun");
    }
}
