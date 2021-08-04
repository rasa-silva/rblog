use std::{fs, path::PathBuf};

use chrono::Utc;

use crate::cli::CmdError;

pub struct CommandNew {
    pub name: String,
    pub source: PathBuf,
}

impl CommandNew {
    pub fn new_article(&self) -> Result<(), CmdError> {
        let now = Utc::now().format("%Y_%m_%d").to_string();
        let sanitized_name = self.name.replace(" ", "_");
        let filename = format!("{}/{}_{}.md", self.source.display(), now, sanitized_name);
        let prefill = format!("# {}\n\n", self.name);

        fs::write(&filename, prefill)?;

        println!("Created {}!", filename);

        Ok(())
    }
}
