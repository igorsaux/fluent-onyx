use fluent::{FluentArgs, FluentValue};
use rand::prelude::SliceRandom;

pub fn pick<'a>(positional: &[FluentValue<'a>], _named: &FluentArgs) -> FluentValue<'a> {
    let mut rng = rand::thread_rng();

    positional
        .choose(&mut rng)
        .unwrap_or(&FluentValue::Error)
        .to_owned()
}
