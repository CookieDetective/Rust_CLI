fn label(site: &str, visits: u32) -> String {
    format!("{site}: {visits} visits")
}

fn main() {
    let site = "example.com";
    let visits = 3;
    println!("{}", label(site, visits));
}

#[cfg(test)]
mod tests {
    use super::label;

    #[test]
    fn formats_values() {
        assert_eq!(label("example.com", 3), "example.com: 3 visits");
    }
}
