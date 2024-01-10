use crate::config;
use crate::requests::{get_latest_article, ping_user, publish_article, update_article, Article};

use anyhow::Result;
use serde::Deserialize;
use spinner::SpinnerBuilder;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub published: bool,
    pub tags: String,
}

pub fn handle_publish_article(article_path: PathBuf, should_update: bool) -> Result<()> {
    let api_key = config::get_api_key()?;
    let sp = SpinnerBuilder::new("Attempting to connect to dev.to...\n".into()).start();

    match ping_user(&api_key) {
        Ok(()) => {
            sp.update("Connected!".into());
            sp.close();
        }
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    }

    let article_content = std::fs::read_to_string(article_path.canonicalize()?)?;
    let [_, frontmatter_str, markdown] =
        article_content.splitn(3, "---").collect::<Vec<&str>>()[..]
    else {
        todo!()
    };
    let frontmatter: FrontMatter = serde_yaml::from_str(frontmatter_str)?;
    let article = Article::with_frontmatter(&frontmatter, markdown);
    let sp_article = SpinnerBuilder::new("Posting your article...\n".into()).start();

    let article_url = if should_update {
        if let Some(latest_article) = get_latest_article(&api_key)? {
            if let Some(id) = latest_article.id {
                update_article(&api_key, article, id)?
            } else {
                String::from("")
            }
        } else {
            String::from("")
        }
    } else {
        publish_article(&api_key, article)?
    };

    sp_article.update("Posted!".into());
    sp_article.close();
    println!(
        "Congrats! Your article has been posted!\nHere's the url: {}",
        article_url
    );

    Ok(())
}
