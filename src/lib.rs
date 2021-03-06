use std::{cell::RefCell, panic};

use backtrace::Backtrace;
use flexi_logger::{FileSpec, Logger, WriteMode};

use byond::byond;
use fluent::{FluentBundle, FluentResource};
use localization_context::LocalizationContext;
use localization_resource::LocalizationResource;
use localizeable_data::LocalizeableData;
use log::{error, info, trace};

mod functions;
mod localization_context;
mod localization_resource;
mod localizeable_data;
mod test_utils;

const NO_TRANSLATION: &str = "NO TRANSLATION";
const ERROR_MESSAGE: &str = "Something is going wrong. Check logs.";

thread_local! {
    pub static CONTEXT: RefCell<LocalizationContext> = RefCell::new(LocalizationContext::new());
}

fn get_inner(json: &str) -> Option<String> {
    trace!("fn get({}: \"{}\")", stringify!(json), json);

    CONTEXT.with(|context| {
        let context = context.borrow();

        let parsed = LocalizeableData::from_str(json).ok()?;
        let result = context.resolve_message(parsed)?;

        trace!("fn get(...) -> {result}");

        Some(result)
    })
}

fn no_translation_log(data: &str) -> String {
    let msg = format!("{NO_TRANSLATION}: {data}");
    error!("{msg}");

    msg
}

byond!(get: json; {
    let got_error = panic::catch_unwind(|| {
        get_inner(json)
    });

    match got_error {
        Ok(message) => match message {
            None => no_translation_log(json),
            Some(v) => if v.is_empty() { no_translation_log(json) } else { v }
        },
        Err(_) => ERROR_MESSAGE.to_string()
    }

});

fn init_inner(localization_folder: &str, fallbacks: &str) {
    let _logger = Logger::try_with_str("trace, fluent-onyx=trace")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("data/logs/fluent/")
                .basename("fluent_onyx"),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    panic::set_hook(Box::new(|_| {
        let bt = Backtrace::new();

        error!("{bt:?}")
    }));

    trace!(
        "fn init({}: \"{}\", {}: \"{}\")",
        stringify!(localization_folder),
        localization_folder,
        stringify!(fallbacks),
        fallbacks
    );

    CONTEXT.with(|context| {
        let mut context = context.borrow_mut();

        let fallbacks = serde_json::from_str(fallbacks).unwrap();

        info!("Fallbacks: {fallbacks:#?}");

        context.set_fallbacks(fallbacks);

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

            context.add_bundle(code, bundle);
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
        Err(_) => ERROR_MESSAGE.to_string()
    }
});
