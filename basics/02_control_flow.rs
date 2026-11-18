fn classify_status(status: u16) -> &'static str {
    if status < 300 {
        "success"
    } else if status < 400 {
        "redirect"
    } else {
        "error"
    }
}

fn main() {
    for status in [200, 301, 404] {
        println!("{status}: {}", classify_status(status));
    }
}

#[cfg(test)]
mod tests {
    use super::classify_status;

    #[test]
    fn groups_status_codes() {
        assert_eq!(classify_status(200), "success");
        assert_eq!(classify_status(301), "redirect");
        assert_eq!(classify_status(404), "error");
    }
}
