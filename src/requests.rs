use crate::commands::FrontMatter;
use crate::errors::ApiKeyError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error;

static USER_ENDPOINT: &str = concat!("https://dev.to/api", "/users/me");
static ARTICLE_ENDPOINT: &str = concat!("https://dev.to/api", "/articles");

#[derive(Deserialize)]
struct UserResp {
    id: Option<i32>,
}

#[derive(Debug, Serialize)]
struct ArticleReq {
    article: Article,
}

#[derive(Debug, Serialize)]
pub struct Article {
    pub id: Option<usize>,
    pub tags: String,
    pub title: String,
    pub published: bool,
    pub body_markdown: String,
}

impl Article {
    pub fn with_frontmatter(matter: &FrontMatter, markdown: &str) -> Self {
        Article {
            id: None,
            tags: matter.tags.clone(),
            title: matter.title.clone(),
            published: matter.published,
            body_markdown: markdown.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ArticleResp {
    id: usize,
    url: String,
    tag_list: Vec<String>,
    title: String,
    published: bool,
    body_markdown: String,
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

pub fn get_latest_article(api_key: &str) -> Result<Option<Article>> {
    let article_resp: Vec<ArticleResp> =
        ureq::get(format!("{}/me/unpublished", ARTICLE_ENDPOINT).as_str())
            .set("api_key", api_key)
            .call()?
            .into_json()?;

    match article_resp.first() {
        Some(&ArticleResp {
            id,
            ref url,
            ref tag_list,
            ref title,
            published,
            ref body_markdown,
        }) => Ok(Some(Article {
            id: Some(id),
            tags: tag_list.join(","),
            title: title.to_string(),
            published: published,
            body_markdown: body_markdown.to_string(),
        })),
        None => Ok(None),
    }
}

pub fn publish_article(api_key: &str, article: Article) -> Result<String> {
    let article_req = ArticleReq { article };
    let article_resp: ArticleResp = ureq::post(ARTICLE_ENDPOINT)
        .set("api_key", api_key)
        .send_json(article_req)?
        .into_json()?;

    Ok(article_resp.url)
}

pub fn update_article(api_key: &str, article: Article, id: usize) -> Result<String> {
    let article_req = ArticleReq { article };

    eprintln!("{:?}", serde_json::to_string(&article_req));

    let article_resp = ureq::put(format!("{}/{}", ARTICLE_ENDPOINT, id).as_str())
        .set("api_key", api_key)
        .send_json(article_req)?;

    let article_resp: ArticleResp = match article_resp.into_json() {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to read JSON: {}", err);
            eprintln!("Response body: {}", err.to_string());
            return Err(err.into());
        }
    };

    Ok(article_resp.url)
}
