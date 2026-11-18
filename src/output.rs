use std::fmt;

use serde::Serialize;

use crate::error::AppError;

pub fn emit<T: Serialize + fmt::Display>(value: &T, json: bool) -> Result<(), AppError> {
    println!("{}", render(value, json)?);
    Ok(())
}

fn render<T: Serialize + fmt::Display>(value: &T, json: bool) -> Result<String, AppError> {
    if json {
        Ok(serde_json::to_string_pretty(value)?)
    } else {
        Ok(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{commands::dns::DnsInfo, output::render};

    #[test]
    fn renders_results_as_json() {
        let result = DnsInfo {
            host: "localhost".to_owned(),
            addresses: vec!["127.0.0.1".parse().expect("valid address")],
        };
        let json = render(&result, true).expect("serialize result");
        assert!(json.contains("\"host\": \"localhost\""));
    }
}
