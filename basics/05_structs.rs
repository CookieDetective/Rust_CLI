struct Site {
    host: String,
    status: u16,
}

impl Site {
    fn is_available(&self) -> bool {
        self.status < 500
    }
}

fn main() {
    let site = Site {
        host: "example.com".to_owned(),
        status: 200,
    };
    println!("{} available: {}", site.host, site.is_available());
}

#[cfg(test)]
mod tests {
    use super::Site;

    #[test]
    fn checks_site_availability() {
        let site = Site {
            host: "example.com".to_owned(),
            status: 503,
        };
        assert!(!site.is_available());
    }
}
