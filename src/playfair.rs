// A simple implementation of the Playfair cipher
use collect_slice::CollectSlice;
use itertools::Itertools;

/// Generates the corresponding Playfair key sequence from a keyword.
/// If J is included,
/// The key is a particular ordering of the alphabet, where the key comes first (except J)
pub fn generate_key(keyword: &str, omit: char) -> Box<[char; 25]> {
    if !keyword.chars().all(|c| ('A'..='Z').contains(&c)) {
        panic!("Keys must be upper-alpha.");
    }
    let mut out = Box::new([' '; 25]);
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".replace(omit, "");
    keyword
        .chars()
        .map(|c| if c == omit {
            panic!("Keyword cannot contain the ommited char.");
        } else { c }) // replace J with I
        .chain(alphabet.chars()) // append alphabet
        .unique() // remove dupes
        .collect_slice_checked(&mut *out);
    out
}
/// Utility: locate the indexes of the key. Will panic if char is not is in key.
fn index_2d(a: char, key: &[char; 25]) -> (usize, usize) {
    let i = key.iter().position(|t| *t == a).unwrap();
    (i / 5, i % 5)
}
/// Transforms a single pair of characters.
/// Will panic if key does not contain chars.
/// Undefined if a == b.
fn playfair_crypt_pair(a: char, b: char, key: &[char; 25], decrypt: bool) -> (char, char) {
    assert_ne!(a, b);
    let delta = if decrypt { 4 } else { 1 };
    // get indexes of each pair
    let (a, b) = {
        let (mut a_row, mut a_col) = index_2d(a, key);
        let (mut b_row, mut b_col) = index_2d(b, key);
        if a_row == b_row {
            a_col = (a_col + delta) % 5; // same row, shift right 1
            b_col = (b_col + delta) % 5;
        } else if a_col == b_col {
            a_row = (a_row + delta) % 5;
            b_row = (b_row + delta) % 5;
        } else {
            (a_col, b_col) = (b_col, a_col); // rectangle, swap columns
        }
        ((a_row, a_col), (b_row, b_col))
    };
    return (key[a.0 * 5 + a.1], key[b.0 * 5 + b.1]); // convert new coords
}
fn crypt(text: &str, key: &str, decrypt: bool, omit: char) -> String {
    // Error checking
    text.chars().for_each(|c| {
        if !('A'..='Z').contains(&c) {
            panic!("Text must be strictly upper-alpha (A-Z). Found '{}'.", c);
        }
    });
    text.chars().tuples::<(_, _)>().for_each(|(a, b)| {
        if a == b {
            panic!("Text must not contain repeated chars. Found '{a}{b}'. Try '{a}X{b}'.");
        }
    });
    let mut text = text.clone().to_string();
    if text.len() % 2 != 0 {
        text.push('Z');
    }
    let key = generate_key(key, omit);
    text.chars()
        .tuples::<(_, _)>()
        .flat_map(|(a, b)| {
            let (a, b) = playfair_crypt_pair(a, b, &key, decrypt);
            [a, b]
        })
        .collect()
}

/// Encrypt the plaintext text with the given key, using the Playfair cipher.
///
/// The key and text must consist of
/// The text must consist of an even number of characters, with none repeated.
/// Panics otherwise.
pub fn encrypt(text: &str, key: &str, omit: char) -> String {
    return crypt(text, key, false, omit);
}
/// Decrypts the plaintext text with the given key, using the Playfair cipher.
///
/// Panics if the input is not all uppercase alphabetical characters.
pub fn decrypt(text: &str, key: &str, omit: char) -> String {
    return crypt(text, key, true, omit);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_key() {
        // combine J and K
        let key: String = generate_key("INTERCEPTOR", 'K').iter().collect();
        assert_eq!(key, "INTERCPOABDFGHJLMQSUVWXYZ");
    }

    #[test]
    fn test_encrypt() {
        let result = encrypt("ZAPXPHOD", "ZERO", 'J');
        assert_eq!(result, "EZSVNIRF");
    }
    
    #[test]
    fn test_decrypt() {
        let result = decrypt("EZSVNIRF", "ZERO", 'J');
        assert_eq!(result, "ZAPXPHOD");
    }
}
