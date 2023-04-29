use crate::commands::FrontMatter;
use crate::errors::ApiKeyError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error;

static USER_ENDPOINT: &str = concat!("https://dev.to/api", "/users/me");
static ARTCLE_ENDPOINT: &str = concat!("https://dev.to/api", "/articles");

#[derive(Deserialize)]
struct UserResp {
    id: Option<i32>,
}

#[derive(Serialize)]
struct ArticleReq {
    article: Article,
}

#[derive(Serialize)]
pub struct Article {
    tags: String,
    title: String,
    published: bool,
    body_markdown: String,
}

impl Article {
    pub fn with_frontmatter(matter: &FrontMatter, markdown: &str) -> Self {
        Article {
            tags: matter.tags.clone(),
            title: matter.title.clone(),
            published: matter.published,
            body_markdown: markdown.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ArticleResp {
    url: String,
}

pub fn ping_user(api_key: &str) -> Result<(), Box<dyn error::Error>> {
    let user_resp: UserResp = ureq::get(USER_ENDPOINT)
        .set("api_key", api_key)
        .call()?
        .into_json()?;

    if user_resp.id.is_none() {
        return Err(Box::new(ApiKeyError::Invalid));
    }

    Ok(())
}

pub fn publish_article(api_key: &str, article: Article) -> Result<String> {
    let article_req = ArticleReq { article };
    let article_resp: ArticleResp = ureq::post(ARTCLE_ENDPOINT)
        .set("api_key", api_key)
        .send_json(article_req)?
        .into_json()?;

    Ok(article_resp.url)
}
