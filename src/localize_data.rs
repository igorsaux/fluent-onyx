use fluent::{FluentArgs, FluentValue};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizeData {
    pub id: String,
    pub code: String,
    args: Option<BTreeMap<String, serde_json::Value>>,
}

impl LocalizeData {
    pub fn from_str(string: &str) -> Self {
        serde_json::from_str(string).unwrap()
    }

    pub fn args(&self) -> Option<FluentArgs> {
        let args = self.args.as_ref()?;
        let mut result = FluentArgs::new();

        for (key, value) in args.iter() {
            let value = match value {
                serde_json::Value::Number(n) => FluentValue::Number(n.as_f64().unwrap().into()),
                serde_json::Value::String(s) => FluentValue::String(s.into()),
                serde_json::Value::Bool(b) => match b {
                    true => FluentValue::String("true".into()),
                    false => FluentValue::String("false".into()),
                },
                _ => panic!("Bad localization argument"),
            };
            result.set(key, value);
        }

        Some(result)
    }
}
