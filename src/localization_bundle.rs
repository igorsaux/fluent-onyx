use std::collections::BTreeMap;

use fluent::{FluentBundle, FluentResource};

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

    pub fn resolve_bundle(&self, code: &str) -> Option<&FluentBundle<FluentResource>> {
        match self.bundles.get(code) {
            None => {
                let fallbacks = match &self.fallbacks_table {
                    None => return None,
                    Some(v) => v,
                };

                return match fallbacks.get(code) {
                    None => None,
                    Some(v) => self.resolve_bundle(v),
                };
            }
            v @ Some(_) => v,
        }
    }
}
