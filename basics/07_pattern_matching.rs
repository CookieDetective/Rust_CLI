fn describe_status(status: u16) -> &'static str {
    match status {
        200..=299 => "successful response",
        300..=399 => "redirect",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown status",
    }
}

fn main() {
    println!("{}", describe_status(200));
}

#[cfg(test)]
mod tests {
    use super::describe_status;

    #[test]
    fn matches_status_ranges() {
        assert_eq!(describe_status(200), "successful response");
        assert_eq!(describe_status(404), "client error");
        assert_eq!(describe_status(700), "unknown status");
    }
}
