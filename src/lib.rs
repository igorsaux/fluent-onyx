use std::{cell::RefCell, collections::BTreeMap, panic};

#[cfg(debug_assertions)]
use backtrace::Backtrace;
#[cfg(debug_assertions)]
use flexi_logger::{FileSpec, Logger, WriteMode};

use byond::byond;
use fluent::{FluentBundle, FluentResource};
use localization_resource::LocalizationResource;
use localize_data::LocalizeData;
use log::{error, info};

mod localization_resource;
mod localize_data;

thread_local! {
    pub static FLUENT_BUNDLES: RefCell<BTreeMap<String, FluentBundle<FluentResource>>> = RefCell::new(BTreeMap::new());
}

byond!(get: json; {
    info!("fn get: {json:#?}");

    FLUENT_BUNDLES.with(|bundles| {
        let bundles = bundles.borrow();
        let parsed = LocalizeData::from_str(json);

        let bundle = bundles.get(&parsed.code).unwrap();
        let message = bundle.get_message(&parsed.id).unwrap();
        let mut errors = Vec::new();
        let args = parsed.args();
        let result = bundle.format_pattern(message.value().unwrap(), args.as_ref(), &mut errors);

        info!("result: {result}");
        error!("errors: {errors:#?}");

        result.to_string()
    })
});

byond!(init: localization_folder; {
    #[cfg(debug_assertions)]
    let _logger = Logger::try_with_str("trace, fluent-onyx=trace").unwrap()
        .log_to_file(FileSpec::default().basename("fluent_onyx"))
        .write_mode(WriteMode::BufferAndFlush)
        .start().unwrap();

    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(|_| {
        let bt = Backtrace::new();

        error!("{bt:?}")
    }));

    info!("Initialized");

    FLUENT_BUNDLES.with(|bundles| {
        let mut bundles = bundles.borrow_mut();

        let entries = std::fs::read_dir(localization_folder).unwrap();

        for entry in entries {
            let entry = entry.unwrap();

            let bundle = match LocalizationResource::from_dir_entry(&entry) {
                None => continue,
                Some(v) => v,
            };


            info!("Loaded bundle: [{}]: \"{}\"", bundle.code(), bundle.path().display());

            let code = bundle.code().to_owned();
            let mut bundle: FluentBundle<FluentResource> = bundle.into();

            bundle.set_use_isolating(false);

            bundles.insert(code, bundle);
        }
    });

    "1"
});
