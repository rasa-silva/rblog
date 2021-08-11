use std::{io, path::PathBuf};

use structopt::StructOpt;

use crate::{cmd_build, cmd_new::CommandNew};

#[derive(StructOpt, Debug)]
#[structopt(about = "An opinionated, configureless SSG.")]
pub enum CliCommand {
    /// Generates the blog using the source dir int output dir.
    Build {
        /// Directory containing the source articles in Markdown.
        #[structopt(default_value = "articles")]
        source: PathBuf,
        /// Directory to put the generated HTML files.
        #[structopt(default_value = "web")]
        output: PathBuf,
    },
    /// Creates a new article.
    ///
    /// Some additional details here...
    New {
        /// Base article name.
        /// A date prefix will be added so the file created will be dd_mm_yyyy_the_article_name.md
        name: String,
        /// Directory containing the source articles in Markdown.
        #[structopt(default_value = "articles")]
        source: PathBuf,
    },
}

impl CliCommand {
    pub fn run(self) -> Result<(), CmdError> {
        match self {
            CliCommand::Build { source, output } => cmd_build::generate_blog(&source, &output),
            CliCommand::New { name, source } => CommandNew { name, source }.new_article(),
        }
    }
}

#[derive(Debug)]
pub enum CmdError {
    Io(io::Error),
    Render(handlebars::RenderError),
    Template(handlebars::TemplateError),
}

impl From<io::Error> for CmdError {
    fn from(error: io::Error) -> Self {
        CmdError::Io(error)
    }
}

impl From<handlebars::TemplateError> for CmdError {
    fn from(error: handlebars::TemplateError) -> Self {
        CmdError::Template(error)
    }
}

impl From<handlebars::RenderError> for CmdError {
    fn from(error: handlebars::RenderError) -> Self {
        CmdError::Render(error)
    }
}
