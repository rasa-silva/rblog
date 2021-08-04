mod article_processor;
mod cli;
mod cmd_build;
mod cmd_new;

use article_processor::ArticleProcessingError;
use cli::{CliCommand, CmdError};
use env_logger::Builder;
use handlebars::TemplateError;
use structopt::StructOpt;

impl From<TemplateError> for ArticleProcessingError {
    fn from(te: TemplateError) -> Self {
        ArticleProcessingError::TemplateError(te)
    }
}

impl From<handlebars::RenderError> for ArticleProcessingError {
    fn from(re: handlebars::RenderError) -> Self {
        ArticleProcessingError::RenderError(re)
    }
}

fn main() -> Result<(), CmdError> {
    Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let command = CliCommand::from_args();
    command.run()
}
