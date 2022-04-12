#![allow(dead_code)]

use fluent::{FluentBundle, FluentResource};

pub fn fast_bundle(code: &str, content: &str) -> FluentBundle<FluentResource> {
    let resource = FluentResource::try_new(content.to_string()).unwrap();
    let mut bundle = FluentBundle::new(vec![code.parse().unwrap()]);

    bundle.add_resource(resource).unwrap();

    bundle
}

pub fn fast_format(bundle: &FluentBundle<FluentResource>, id: &str) -> String {
    let message = bundle.get_message(id).unwrap();
    let mut errors = Vec::new();

    bundle
        .format_pattern(message.value().unwrap(), None, &mut errors)
        .to_string()
}
