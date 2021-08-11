use crate::article_processor::*;
use crate::cli::CmdError;
use handlebars::Handlebars;
use log::info;
use log::warn;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn generate_blog(source: &Path, output: &Path) -> Result<(), CmdError> {
    fs::remove_dir_all(&output)?;
    fs::create_dir_all(&output)?;
    let mut articles = walk_article_dir(source.to_str().unwrap())?;
    make_index_page(output, &mut articles)?;
    generate_search_index(output, &articles)?;
    Ok(())
}

fn walk_article_dir(path: &str) -> Result<Vec<Article>, CmdError> {
    let mut articles = Vec::new();
    let article_processor = ArticleProcessor::build().expect("Failed to build processor.");

    for elem in fs::read_dir(path)? {
        let path_buf = elem?.path();
        let path = path_buf.as_path();
        let filename = path.file_name().expect("Invalid filename");
        if path.is_dir() {
            warn!("Not recursing into {:?}", filename)
        } else {
            info!("Processing file: {:?}", filename);

            match article_processor.process_article(path) {
                Ok(article) => articles.push(article),
                Err(err) => println!("Error processing {:?}: {}", filename, err),
            }
        }
    }

    Ok(articles)
}

fn make_index_page(output: &Path, articles: &mut Vec<Article>) -> Result<(), CmdError> {
    info!("Generating index page...");

    //make the index links on reverse chronological order (assume filename gives this)
    articles.sort_by(|a1, a2| a1.filename.cmp(&a2.filename).reverse());

    let mut reg = Handlebars::new();
    reg.register_template_file("index_template", Path::new("index_template.hbs"))?;

    let mut data = HashMap::new(); //TODO do we really need to create this?
    data.insert("articles", articles);

    let rendered = reg.render("index_template", &data)?;

    std::fs::write(format!("{}/index.html", output.display()), rendered)?;

    Ok(())
}

fn generate_search_index(output: &Path, articles: &[Article]) -> Result<(), CmdError> {
    //TODO the article structure needs to include the link field to match the expected
    //json format
    let index_contents = serde_json::to_string(articles).unwrap();
    std::fs::write(format!("{}/search_data.json", output.display()), index_contents)?;
    Ok(())
}

