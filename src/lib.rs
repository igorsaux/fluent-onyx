use std::{cell::RefCell, panic};

#[cfg(debug_assertions)]
use backtrace::Backtrace;
#[cfg(debug_assertions)]
use flexi_logger::{FileSpec, Logger, WriteMode};

use byond::byond;
use fluent::{FluentBundle, FluentResource};
use localization_bundle::LocalizationBundles;
use localization_resource::LocalizationResource;
use localizeable_data::LocalizeableData;
use log::{error, info};

mod localization_bundle;
mod localization_resource;
mod localizeable_data;

const ERROR_MESSAGE: &str = "NO TRANSLATION";

thread_local! {
    pub static WRAPPER: RefCell<LocalizationBundles> = RefCell::new(LocalizationBundles::new());
}

fn get_inner(json: &str) -> Option<String> {
    info!("fn get: {json:#?}");

    WRAPPER.with(|wrapper| {
        let wrapper = wrapper.borrow();

        let parsed = LocalizeableData::from_str(json).ok()?;
        let bundle = wrapper.resolve_bundle(&parsed.code)?;
        let message = bundle.get_message(&parsed.id)?;
        let value = message.value()?;

        let mut errors = Vec::new();
        let args = parsed.args();
        let result = bundle.format_pattern(value, args.as_ref(), &mut errors);

        info!("result: {result}");
        error!("errors: {errors:#?}");

        Some(result.to_string())
    })
}

byond!(get: json; {
    match get_inner(json) {
        Some(v) => v,
        None => ERROR_MESSAGE.into()
    }
});

fn init_inner(localization_folder: &str, fallbacks: &str) -> Option<String> {
    #[cfg(debug_assertions)]
    let _logger = Logger::try_with_str("trace, fluent-onyx=trace")
        .ok()?
        .log_to_file(
            FileSpec::default()
                .directory("data/logs/")
                .basename("fluent_onyx"),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .ok()?;

    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(|_| {
        let bt = Backtrace::new();

        error!("{bt:?}")
    }));

    info!("Initialized");

    WRAPPER.with(|wrapper| {
        let mut wrapper = wrapper.borrow_mut();

        let fallbacks = serde_json::from_str(fallbacks).ok()?;

        info!("Fallbacks: {fallbacks:#?}");

        wrapper.set_fallbacks(fallbacks);

        let bundles = wrapper.bundles_mut();

        let entries = std::fs::read_dir(localization_folder).ok()?;

        for entry in entries {
            let entry = entry.ok()?;
            let bundle = LocalizationResource::from_dir_entry(&entry)?;

            info!(
                "Loaded bundle: [{}]: \"{}\"",
                bundle.code(),
                bundle.path().display()
            );

            let code = bundle.code().to_owned();
            let mut bundle: FluentBundle<FluentResource> = bundle.try_into().ok()?;

            bundle.set_use_isolating(false);

            bundles.insert(code, bundle);
        }

        Some("".to_string())
    })
}

byond!(init: localization_folder, fallbacks; {
    match init_inner(localization_folder, fallbacks) {
        Some(v) => v,
        None => ERROR_MESSAGE.into()
    }
});
