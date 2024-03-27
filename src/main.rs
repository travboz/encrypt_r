use encrypt_r::{self, encrypt_file, get_user_input, EncrypterConfig};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let enc = EncrypterConfig::new()?;

    // let input = get_user_input("Enter a string you wish to encrypt: ")?;

    let input_path = "fox1.txt";
    let (encrypted_file_path, key_path) = encrypt_file(input_path, &enc)?;

    // let input_d = Encrypter::get_decrypt_path();
    // let input_k = Encrypter::get_key_path();

    // let decrypted_file_path = Encrypter::decrypt_file(file, d_key);

    // // encrypt input
    // let encrypted_input = enc.encrypt(&input)?;
    // println!("Encrypted text is now: {encrypted_input}");

    // // Decrypt input
    // let decrypted_input = enc.decrypt(&encrypted_input)?;
    // println!("Original text is now: {decrypted_input}");

    Ok(())
}
