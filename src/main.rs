use encrypt_r::{self, decrypt_file, encrypt_file, get_key, get_user_input, EncrypterConfig};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut enc = EncrypterConfig::new()?;

    // let input = get_user_input("Enter a string you wish to encrypt: ")?;

    let input_path = "lorum.txt";
    let (encrypted_file_path, key_path) = encrypt_file(input_path, &enc)?;

    println!("Encrypted file in: '{encrypted_file_path}'");
    println!("Key file in: '{key_path}'");

    let input_path = "encrypted_lorum.txt";
    let k_p = "key_lorum.txt";

    let d = decrypt_file(input_path, k_p, &mut enc)?;
    println!("Decrypted file in: '{}'", d);

    Ok(())
}
