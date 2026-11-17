fn parse_port(value: &str) -> Result<u16, std::num::ParseIntError> {
    value.parse()
}

fn main() {
    match parse_port("443") {
        Ok(port) => println!("Port: {port}"),
        Err(error) => eprintln!("Invalid port: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn parses_valid_ports_and_reports_invalid_values() {
        assert_eq!(parse_port("443"), Ok(443));
        assert!(parse_port("not-a-port").is_err());
    }
}
