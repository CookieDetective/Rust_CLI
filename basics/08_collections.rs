use std::collections::HashSet;

fn unique_hosts(hosts: &[&str]) -> HashSet<String> {
    hosts.iter().map(|host| host.to_ascii_lowercase()).collect()
}

fn main() {
    println!("Unique hosts: {}", unique_hosts(&["example.com", "EXAMPLE.com"]).len());
}

#[cfg(test)]
mod tests {
    use super::unique_hosts;

    #[test]
    fn stores_case_insensitive_unique_hosts() {
        assert_eq!(unique_hosts(&["example.com", "EXAMPLE.com"]).len(), 1);
    }
}
