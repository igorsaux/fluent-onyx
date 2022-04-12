use fluent::{FluentArgs, FluentBundle, FluentError, FluentResource, FluentValue};
use rand::prelude::SliceRandom;
mod en;

pub fn pick<'a>(positional: &[FluentValue<'a>], _named: &FluentArgs) -> FluentValue<'a> {
    let mut rng = rand::thread_rng();

    positional
        .choose(&mut rng)
        .unwrap_or(&FluentValue::Error)
        .to_owned()
}

pub fn add_functions(bundle: &mut FluentBundle<FluentResource>) -> Result<(), FluentError> {
    bundle.add_function("PICK", pick)?;
    bundle.add_function("THE", en::the)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fast_bundle, fast_format};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pick() {
        let mut bundle = fast_bundle(
            "ru",
            r#"
foo = { PICK("one") }
"#,
        );

        bundle.add_function("PICK", super::pick).unwrap();

        let result = fast_format(&bundle, "foo");

        assert_eq!(result, "one")
    }
}
