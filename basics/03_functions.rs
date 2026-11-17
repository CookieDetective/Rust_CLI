fn average(samples: &[u32]) -> Option<f64> {
    if samples.is_empty() {
        return None;
    }
    let total: u32 = samples.iter().sum();
    Some(f64::from(total) / samples.len() as f64)
}

fn main() {
    println!("Average: {:?}", average(&[200, 204, 500]));
}

#[cfg(test)]
mod tests {
    use super::average;

    #[test]
    fn calculates_average_and_handles_no_samples() {
        assert_eq!(average(&[2, 4]), Some(3.0));
        assert_eq!(average(&[]), None);
    }
}
