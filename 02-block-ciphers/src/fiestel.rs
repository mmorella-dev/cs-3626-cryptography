type Byte = u8;
pub type Block = (Byte, Byte);
pub type Subkey = Byte;
pub type Key = [Subkey];

/// Maps some byte to some other byte.
fn round_function(u: Byte, k: Subkey) -> Byte {
    u.wrapping_add(k).reverse_bits().wrapping_neg()
}

fn fiestel_cipher<'a, It>(b: Block, key: It) -> Block
where
    It: Iterator<Item = &'a Subkey> + 'a,
{
    let (mut l, mut r) = b;
    for &subkey in key {
        (l, r) = (r, l ^ round_function(r, subkey));
    }
    return (r, l);
}

pub fn encrypt_block(b: Block, key: &Key) -> Block {
    fiestel_cipher(b, key.iter())
}
/// Performs the same call as encrypt, but in reverse.
pub fn decrypt_block(b: Block, key: &Key) -> Block {
    fiestel_cipher(b, key.iter().rev())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_block1() {
        let key = b"Hello";
        let plain = (b'A', b'X');
        let enciphered = encrypt_block(plain, key);
        assert_ne!(plain, enciphered);
        let deciphered = decrypt_block(enciphered, key);
        assert_eq!(plain, deciphered);
    }
    #[test]
    fn test_encrypt_block2() {
        let key = b"aaa";
        let plain = (b'a', b'Y');
        let enciphered = encrypt_block(plain, key);
        assert_ne!(plain, enciphered);
        let deciphered = decrypt_block(enciphered, key);
        assert_eq!(plain, deciphered);
    }
}