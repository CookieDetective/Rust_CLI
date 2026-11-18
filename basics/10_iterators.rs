fn successful_statuses(statuses: &[u16]) -> Vec<u16> {
    statuses
        .iter()
        .copied()
        .filter(|status| (200..300).contains(status))
        .collect()
}

fn main() {
    println!("{:?}", successful_statuses(&[200, 301, 204, 500]));
}

#[cfg(test)]
mod tests {
    use super::successful_statuses;

    #[test]
    fn filters_successful_statuses() {
        assert_eq!(successful_statuses(&[200, 301, 204, 500]), [200, 204]);
    }
}
