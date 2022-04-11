use std::{cell::RefCell, panic};

use backtrace::Backtrace;
use flexi_logger::{FileSpec, Logger, WriteMode};

use byond::byond;
use fluent::{FluentBundle, FluentResource};
use localization_bundle::LocalizationBundles;
use localization_resource::LocalizationResource;
use localizeable_data::LocalizeableData;
use log::{error, info, trace};

mod localization_bundle;
mod localization_resource;
mod localizeable_data;

const ERROR_MESSAGE: &str = "NO TRANSLATION";

thread_local! {
    pub static WRAPPER: RefCell<LocalizationBundles> = RefCell::new(LocalizationBundles::new());
}

fn get_inner(json: &str) -> Option<String> {
    #[cfg(debug_assertions)]
    trace!("fn get({}: \"{}\")", stringify!(json), json);

    WRAPPER.with(|wrapper| {
        let wrapper = wrapper.borrow();

        let parsed = LocalizeableData::from_str(json).ok()?;
        let bundle = wrapper.resolve_bundle(&parsed.code)?;
        let message = bundle.get_message(&parsed.id)?;
        let value = message.value()?;

        let mut errors = Vec::new();
        let args = parsed.args();
        let result = bundle.format_pattern(value, args.as_ref(), &mut errors);

        #[cfg(debug_assertions)]
        trace!("fn get(...) -> {result}");

        if !errors.is_empty() {
            error!("errors: {errors:#?}");
        }

        Some(result.to_string())
    })
}

byond!(get: json; {
    match get_inner(json) {
        Some(v) => v,
        None => ERROR_MESSAGE.into()
    }
});

fn init_inner(localization_folder: &str, fallbacks: &str) {
    let _logger = Logger::try_with_str("trace, fluent-onyx=trace")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("data/logs/")
                .basename("fluent_onyx"),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    panic::set_hook(Box::new(|_| {
        let bt = Backtrace::new();

        error!("{bt:?}")
    }));

    #[cfg(debug_assertions)]
    trace!(
        "fn init({}: \"{}\", {}: \"{}\")",
        stringify!(localization_folder),
        localization_folder,
        stringify!(fallbacks),
        fallbacks
    );

    WRAPPER.with(|wrapper| {
        let mut wrapper = wrapper.borrow_mut();

        let fallbacks = serde_json::from_str(fallbacks).unwrap();

        info!("Fallbacks: {fallbacks:#?}");

        wrapper.set_fallbacks(fallbacks);

        let bundles = wrapper.bundles_mut();

        let entries = std::fs::read_dir(localization_folder).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let bundle = match LocalizationResource::from_dir_entry(&entry) {
                None => continue,
                Some(v) => v,
            };

            info!(
                "Loaded bundle: [{}]: \"{}\"",
                bundle.code(),
                bundle.path().display()
            );

            let code = bundle.code().to_owned();
            let mut bundle: FluentBundle<FluentResource> = bundle.try_into().unwrap();

            bundle.set_use_isolating(false);

            bundles.insert(code, bundle);
        }

        info!("Initialized");
    })
}

byond!(init: localization_folder, fallbacks; {
    let got_error = panic::catch_unwind(|| {
        init_inner(localization_folder, fallbacks)
    });

    match got_error {
        Ok(()) => "".to_string(),
        Err(_) => "Something is going wrong. Check logs for information.".to_string()
    }
});
