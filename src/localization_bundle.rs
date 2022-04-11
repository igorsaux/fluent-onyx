use std::collections::BTreeMap;

use fluent::{FluentBundle, FluentMessage, FluentResource};
use log::{error, info};

use crate::localizeable_data::LocalizeableData;

type Bundles = BTreeMap<String, FluentBundle<FluentResource>>;
type FallbacksTable = BTreeMap<String, String>;

#[derive(Default)]
pub struct LocalizationBundles {
    bundles: Bundles,
    fallbacks_table: Option<FallbacksTable>,
}

impl LocalizationBundles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bundles(&self) -> &Bundles {
        &self.bundles
    }

    pub fn bundles_mut(&mut self) -> &mut Bundles {
        &mut self.bundles
    }

    pub fn fallbacks(&self) -> Option<&FallbacksTable> {
        self.fallbacks_table.as_ref()
    }

    pub fn set_fallbacks(&mut self, fallbacks: FallbacksTable) {
        self.fallbacks_table = Some(fallbacks);
    }

    fn format_message(
        &self,
        bundle: &FluentBundle<FluentResource>,
        message: &FluentMessage,
        data: &LocalizeableData,
    ) -> String {
        let value = message.value().expect("Can't get FluentMessage's value");
        let args = data.parse_args();
        let mut errors = Vec::new();

        let message = bundle.format_pattern(value, args.as_ref(), &mut errors);

        if !errors.is_empty() {
            error!("{errors:#?}");
        };

        message.to_string()
    }

    pub fn resolve_message(&self, data: LocalizeableData) -> Option<String> {
        let bundle = self.resolve_bundle(&data.code)?;

        let message = match bundle.get_message(&data.id) {
            None => {
                let table = match &self.fallbacks_table {
                    None => return None,
                    Some(v) => v,
                };

                let new_code = match table.get(&data.code) {
                    None => return None,
                    Some(v) => v,
                };

                let data = LocalizeableData {
                    code: new_code.to_owned(),
                    ..data
                };

                self.resolve_message(data)
            }
            Some(v) => Some(self.format_message(bundle, &v, &data)),
        };

        message
    }

    pub fn resolve_bundle(&self, code: &str) -> Option<&FluentBundle<FluentResource>> {
        match self.bundles.get(code) {
            None => {
                let table = match &self.fallbacks_table {
                    None => return None,
                    Some(v) => v,
                };

                return match table.get(code) {
                    None => None,
                    Some(v) => self.resolve_bundle(v),
                };
            }
            v @ Some(_) => v,
        }
    }
}
