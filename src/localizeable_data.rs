use fluent::{FluentArgs, FluentValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalizeableDataError {
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizeableData {
    pub id: String,
    pub code: String,
    args: Option<BTreeMap<String, serde_json::Value>>,
}

impl LocalizeableData {
    pub fn from_str(string: &str) -> Result<Self, LocalizeableDataError> {
        serde_json::from_str(string).map_err(LocalizeableDataError::Parse)
    }

    pub fn args(&self) -> Option<FluentArgs> {
        let args = self.args.as_ref()?;
        let mut result = FluentArgs::new();

        for (key, value) in args.iter() {
            let value = match value {
                serde_json::Value::Number(n) => FluentValue::Number(n.as_f64()?.into()),
                serde_json::Value::String(s) => FluentValue::String(s.into()),
                serde_json::Value::Bool(b) => match b {
                    true => FluentValue::String("true".into()),
                    false => FluentValue::String("false".into()),
                },
                _ => continue,
            };
            result.set(key, value);
        }

        Some(result)
    }
}
