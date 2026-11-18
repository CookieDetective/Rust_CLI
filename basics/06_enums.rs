enum Protocol {
    Http,
    Https,
}

impl Protocol {
    fn default_port(&self) -> u16 {
        match self {
            Self::Http => 80,
            Self::Https => 443,
        }
    }
}

fn main() {
    println!("HTTPS port: {}", Protocol::Https.default_port());
}

#[cfg(test)]
mod tests {
    use super::Protocol;

    #[test]
    fn maps_protocols_to_default_ports() {
        assert_eq!(Protocol::Http.default_port(), 80);
        assert_eq!(Protocol::Https.default_port(), 443);
    }
}
