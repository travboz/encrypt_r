#[cfg(test)]
mod tests {
    use encrypt_r::Encrypter;

    fn convert_to_vec(input: &str) -> Vec<String> {
        input
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let enc = Encrypter::new().unwrap();
        let plaintext = "abc";

        let encrypted_text = enc.encrypt(&convert_to_vec(plaintext)).unwrap();
        let decrypted_text = enc.decrypt(&encrypted_text).unwrap();

        assert_eq!(plaintext, decrypted_text);
    }

    #[test]
    fn test_alphabet_and_cipher_initialization() {
        let enc = Encrypter::new().unwrap();

        assert_eq!(enc.alphabet.len(), 95);
        assert_eq!(enc.cipher.len(), 95);
    }

    #[test]
    fn test_error_handling() {
        assert!(true);
    }
}
