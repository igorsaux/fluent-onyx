use std::{
    fs::DirEntry,
    io::Read,
    path::{Path, PathBuf},
};

use fluent::{FluentBundle, FluentResource};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalizationResourceError {
    #[error("{0}")]
    Generic(#[from] anyhow::Error),
}

pub struct LocalizationResource {
    file_path: PathBuf,
    code: String,
    content: String,
}

impl LocalizationResource {
    pub fn from_dir_entry(entry: &DirEntry) -> Option<Self> {
        let meta = entry.metadata().ok()?;

        if meta.is_dir() {
            return None;
        }

        if entry.path().extension()? != "ftl" {
            return None;
        }

        let code = entry.path().file_stem()?.to_string_lossy().to_string();
        let mut content = String::new();

        {
            let mut file = std::fs::File::open(entry.path()).ok()?;
            file.read_to_string(&mut content).ok()?;
        }

        Some(Self {
            file_path: entry.path(),
            code,
            content,
        })
    }

    pub fn path(&self) -> &Path {
        &self.file_path
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl TryFrom<LocalizationResource> for FluentBundle<FluentResource> {
    type Error = LocalizationResourceError;

    fn try_from(val: LocalizationResource) -> Result<Self, Self::Error> {
        let resource = FluentResource::try_new(val.content)
            .map_err(|_| anyhow::anyhow!("FluentResource error"))?;
        let mut bundle = FluentBundle::new(vec![val
            .code
            .parse()
            .map_err(|_| anyhow::anyhow!("Parse error"))?]);

        bundle
            .add_resource(resource)
            .map_err(|_| anyhow::anyhow!("FluentBundle error"))?;

        Ok(bundle)
    }
}
