use encrypt_r::{self, Encrypter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let enc = Encrypter::new()?;

    let input = Encrypter::get_user_input("Enter a string you wish to encrypt: ")?;

    // encrypt input
    let encrypted_input = enc.encrypt(&input)?;
    println!("Encrypted text is now: {encrypted_input}");

    // Decrypt input
    let decrypted_input = enc.decrypt(&encrypted_input)?;
    println!("Original text is now: {decrypted_input}");

    Ok(())
}
