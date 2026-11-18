use std::io::Read;

use reqwest::{Url, blocking::Client};

use crate::error::AppError;

const MAX_RESPONSE_BODY_BYTES: u64 = 1_048_576;

#[derive(Debug)]
pub struct PageResponse {
    pub url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

pub fn get(url: &str) -> Result<PageResponse, AppError> {
    let parsed =
        Url::parse(url).map_err(|error| AppError::InvalidInput(format!("invalid URL: {error}")))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(AppError::InvalidInput(
            "URL scheme must be http or https".to_owned(),
        ));
    }

    let response = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?
        .get(parsed)
        .send()?;
    let url = response.url().to_string();
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let mut body = Vec::new();
    response
        .take(MAX_RESPONSE_BODY_BYTES)
        .read_to_end(&mut body)
        .map_err(AppError::Io)?;
    let body = String::from_utf8_lossy(&body);

    Ok(PageResponse {
        url,
        status,
        content_type,
        title: parse_title(&body),
    })
}

fn parse_title(html: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let mut search_from = 0;
    while let Some(relative_start) = lower[search_from..].find("<title") {
        let start = search_from + relative_start;
        let tag_end = start + "<title".len();
        let next = lower.as_bytes().get(tag_end).copied()?;
        if next.is_ascii_whitespace() || next == b'>' {
            let content_start = lower[tag_end..].find('>')? + tag_end + 1;
            let content_end = lower[content_start..].find("</title")? + content_start;
            let title = html[content_start..content_end].trim();
            return (!title.is_empty()).then(|| title.to_owned());
        }
        search_from = tag_end;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::parse_title;

    #[test]
    fn extracts_a_title_case_insensitively() {
        assert_eq!(
            parse_title("<HTML><TITLE> A useful page </TITLE></HTML>"),
            Some("A useful page".to_owned())
        );
    }

    #[test]
    fn ignores_title_like_text_and_empty_titles() {
        assert_eq!(parse_title("<title-card>not a title</title-card>"), None);
        assert_eq!(parse_title("<title>  </title>"), None);
    }
}
