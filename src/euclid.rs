/// Calculates the greatest common denominator of a and b.
///
/// Time complexity: O(log(min(a, b))
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    // euclid's algorithm
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}
/// Calculates the result of the extended Euclidean algorithm.
///
/// Returns (d, x, y) such that ax + by = d = gcd(a,b)
pub fn gcd_extended(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut d, mut d2) = (a, b);
    let (mut x, mut x2) = (1, 0);
    let (mut y, mut y2) = (0, 1);
    while d2 != 0 {
        let q = d / d2;
        (d, d2) = (d2, d - q * d2);
        (x, x2) = (x2, x - q * x2);
        (y, y2) = (y2, y - q * y2);
    }
    return (d, x, y);
}

#[cfg(test)]
mod test_euclid {
    use super::*;
    #[test]
    fn test_gcd1() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(12, 8), 4);
    }
    #[test]
    fn test_gcd2() {
        // check
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(24, 54), 6);
    }
    #[test]
    fn test_gcd3() {
        // Defn: gcd(a, 0) = gcd(0, a) = |a|
        assert_eq!(gcd(0, 15), 15);
        assert_eq!(gcd(15, 0), 15);
    }
    #[test]
    fn test_gcd4() {
        // Defn: gcd(0, 0) = 0
        assert_eq!(gcd(0, 0), 0);
    }
    #[test]
    fn test_gcd_extended1() {
        assert_eq!(gcd_extended(240, 46), (2, -9, 47));
    }
    #[test]
    fn test_gcd_extended2() {
        assert_eq!(gcd_extended(15, 0), (15, 1, 0));
    }
}
