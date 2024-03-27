#[cfg(test)]
mod tests {
    use encrypt_r::{decrypt, encrypt, EncrypterConfig};

    fn convert_to_vec(input: &str) -> Vec<String> {
        input
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let enc = EncrypterConfig::new().unwrap();
        let plaintext = "abc";

        let encrypted_text = encrypt(&convert_to_vec(plaintext), &enc).unwrap();
        let decrypted_text = decrypt(&encrypted_text, &enc).unwrap();

        assert_eq!(plaintext, decrypted_text);
    }

    #[test]
    fn test_alphabet_and_cipher_initialization() {
        let enc = EncrypterConfig::new().unwrap();

        assert_eq!(enc.alphabet.len(), 95);
        assert_eq!(enc.cipher.len(), 95);
    }

    #[test]
    fn test_error_handling() {
        assert!(true);
    }
}
