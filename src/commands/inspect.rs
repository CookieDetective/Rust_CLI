use std::fmt;

use serde::Serialize;

use crate::{error::AppError, http};

#[derive(Debug, Serialize)]
pub struct SiteInfo {
    pub url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

impl fmt::Display for SiteInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "URL: {}", self.url)?;
        writeln!(formatter, "Status: {}", self.status)?;
        writeln!(
            formatter,
            "Content type: {}",
            self.content_type.as_deref().unwrap_or("unknown")
        )?;
        write!(
            formatter,
            "Title: {}",
            self.title.as_deref().unwrap_or("not found")
        )
    }
}

pub fn run(url: &str) -> Result<SiteInfo, AppError> {
    let response = http::get(url)?;
    Ok(SiteInfo {
        url: response.url,
        status: response.status,
        content_type: response.content_type,
        title: response.title,
    })
}
