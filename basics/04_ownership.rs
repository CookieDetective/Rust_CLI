fn normalize_host(host: String) -> String {
    host.trim().to_ascii_lowercase()
}

fn main() {
    let host = String::from(" Example.COM ");
    println!("{}", normalize_host(host));
}

#[cfg(test)]
mod tests {
    use super::normalize_host;

    #[test]
    fn consumes_and_normalizes_a_string() {
        assert_eq!(normalize_host(String::from(" Example.COM ")), "example.com");
    }
}
