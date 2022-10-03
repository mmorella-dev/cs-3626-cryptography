/// The alphabet to use. [a-zA-Z]
pub const ALPHA_52: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub(crate) fn shift(c: char, rot: i32, alpha: &str) -> char {
    let len = alpha.len() as i32;
    // Search for c in alpha
    if let Some(i) = alpha.find(c) {
        // Take the positive result of i + rot mod len
        let i = (i as i32 + rot).rem_euclid(len);
        return alpha.chars().nth(i as usize).unwrap();
    } else {
        return c; // leave character as-is
    }
}

/// Encrypts `text` using a Caesar cipher with a **right-shift** of `rot`.
/// Invoking `decrypt` will return the inverse of invoking `encrypt` with the same args.
/// The ordering of characters is provided by `alpha`. Characters not in `alpha` are left as-is.
pub fn encrypt(text: &str, rot: i32, alpha: &str) -> String {
    text.chars().map(|c| shift(c, rot, alpha)).collect()
}
/// Decrypt `text` using a Caesar cipher with a **left-shift** of `rot`.
/// Invoking `decrypt` will return the inverse of invoking `encrypt` with the same args.
/// The ordering of characters is provided by `alpha`. Characters not in `alpha` are left as-is.
pub fn decrypt(text: &str, rot: i32, alpha: &str) -> String {
    return encrypt(text, -rot, alpha);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shift() {
        assert_eq!(shift('a', 1, ALPHA_52), 'b');
        assert_eq!(shift('A', 1, ALPHA_52), 'B');
        assert_eq!(shift('Z', 1, ALPHA_52), 'a');
        assert_eq!(shift('a', -1, ALPHA_52), 'Z');
        assert_eq!(shift('z', 1, ALPHA_52), 'A');
    }
    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("aAZaz", 1, ALPHA_52), "bBabA");
        assert_eq!(
            encrypt("Hello there!", -10, ALPHA_52),
            "xUbbe jXUhU!"
        );
    }
    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("bBabA", 1, ALPHA_52), "aAZaz");
        assert_eq!(
            decrypt("xUbbe jXUhU!", -10, ALPHA_52),
            "Hello there!"
        );
    }
}