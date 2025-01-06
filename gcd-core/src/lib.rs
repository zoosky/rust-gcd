pub mod gcd {
    pub fn compute_gcd(mut n: u64, mut m: u64) -> u64 {
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            m = m % n;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::gcd::compute_gcd;

    #[test]
    fn test_gcd_computation() {
        assert_eq!(compute_gcd(48, 18), 6);
        assert_eq!(compute_gcd(54, 24), 6);
        assert_eq!(compute_gcd(7, 13), 1);
    }

    #[test]
    #[should_panic]
    fn test_gcd_zero_input() {
        compute_gcd(0, 10);
    }
}
