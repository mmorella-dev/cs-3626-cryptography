/// Alphabet: [a-zA-Z0-9 ]
pub const ALPHA_61: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
pub const ALPHA_52: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn shift(c: char, k: char, alpha: &str, diff: bool) -> char {
    let len = alpha.len() as i32;
    // Search for c in alpha
    if let Some(ci) = alpha.find(c) {
        if let Some(ki) = alpha.find(k) {
            let rot = if diff { -(ki as i32) } else { ki as i32 };
            let i = (ci as i32 + rot).rem_euclid(len);
            return alpha.chars().nth(i as usize).unwrap();
        } else {
            panic!("Input key must consist of characters in '{}'", alpha);
        }
    } else {
        return c; // leave character as-is
    }
}
fn vignere_cipher(text: &str, key: &str, alpha: &str, decrypt: bool) -> String {
    // Pair each char in text with key. Loop key if too short.
    text.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| shift(c, k, alpha, decrypt)) // Sum each pair
        .collect::<String>()
}
/// Encrypts text.
pub fn encrypt(text: &str, key: &str, alpha: &str) -> String {
    vignere_cipher(text, key, alpha, false)
}
/// Decrypts text.
pub fn decrypt(text: &str, key: &str, alpha: &str) -> String {
    vignere_cipher(text, key, alpha, true)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_encrypt1() {
        assert_eq!(
            encrypt("ATTACKATDAWN", "LEMON", ALPHA_UPPER),
            "LXFOPVEFRNHR"
        );
        assert_eq!(
            decrypt("LXFOPVEFRNHR", "LEMON", ALPHA_UPPER),
            "ATTACKATDAWN"
        );
    }
    #[test]
    fn test_encrypt2() {
        assert_eq!(
            encrypt("Hello there.", "QAoom UwAOA.", ALPHA_52),
            "xEzzA nDEfE."
        );
        assert_eq!(
            decrypt("xEzzA nDEfE.", "QAoom UwAOA.", ALPHA_52),
            "Hello there."
        );
    }
    #[test]
    fn test_encrypt3() {
        // note that spaces are still matched with a value in the key
        // the expected value is not expecting this
        assert_eq!(
            encrypt("tokyotower.", "key", ALPHA_LOWER),
            "dsiisryacb."
        );
        assert_eq!(
            decrypt("dsiisryacb.", "key", ALPHA_LOWER),
            "tokyotower."
        );
        
    }
}
