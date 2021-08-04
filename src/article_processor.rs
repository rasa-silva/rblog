use chrono::{DateTime, Utc};
use handlebars::{Handlebars, TemplateError};
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::{self, Path},
};

#[derive(Debug, Serialize)]
pub struct Article {
    title: String,
    body: String,
    created_at: String,
    pub filename: String,
}

pub struct ArticleProcessor<'ap> {
    template_registry: Handlebars<'ap>,
}

#[derive(Debug)]
pub enum ArticleProcessingError {
    CannotReadFile(std::io::Error),
    CannotReadMetadata(std::io::Error),
    CannotCreate(std::io::Error),
    CannotWrite(std::io::Error),
    RenderError(handlebars::RenderError),
    TemplateError(handlebars::TemplateError),
}

impl Error for ArticleProcessingError {}

impl Display for ArticleProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self)
    }
}

impl<'a> ArticleProcessor<'a> {
    pub fn build() -> Result<ArticleProcessor<'a>, TemplateError> {
        let mut reg = Handlebars::new();
        reg.register_escape_fn(handlebars::no_escape); //No escaping for the html body
        reg.register_template_file("article_template", Path::new("article_template.hbs"))?;

        Ok(ArticleProcessor {
            template_registry: reg,
        })
    }

    pub fn process_article(
        &self,
        article_path: &path::Path,
    ) -> Result<Article, ArticleProcessingError> {
        let text =
            fs::read_to_string(article_path).map_err(ArticleProcessingError::CannotReadFile)?;
        let first_line = text.lines().next().unwrap();
        let title = first_line.get(2..).unwrap();
        let created = fs::metadata(article_path)
            .map_err(ArticleProcessingError::CannotReadMetadata)?
            .created()
            .map_err(ArticleProcessingError::CannotReadMetadata)?;

        let t = DateTime::<Utc>::from(created);
        let file_stem = article_path
            .file_stem()
            .expect("Invalid filename")
            .to_str()
            .unwrap();

        let article = Article {
            title: title.to_owned(),
            body: text.to_owned(),
            created_at: t.format("%d/%m/%y").to_string(),
            filename: file_stem.to_string(),
        };

        //Convert markdown to html
        let parser = Parser::new_ext(text.as_str(), Options::empty());
        let mut output_string = String::new();
        html::push_html(&mut output_string, parser);

        //Include the html into the article template
        let mut data = HashMap::new(); //TODO do we really need to create this?
        data.insert("body", output_string);
        data.insert("title", article.title.clone());
        let rendered = self.template_registry.render("article_template", &data)?;

        let output_path = format!("web/{}.html", file_stem);
        let out_path = Path::new(output_path.as_str());
        let mut output = File::create(out_path).map_err(ArticleProcessingError::CannotCreate)?;

        File::write_all(&mut output, rendered.as_bytes())
            .map_err(ArticleProcessingError::CannotWrite)?;

        Ok(article)
    }
}
