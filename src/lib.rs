// default alphabet
const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const DIGITS: &str = "0123456789";
const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPACE: &str = " ";

use rand::rngs::mock::StepRng;
use rand::Rng;

use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::{fs, io};

pub struct EncrypterConfig {
    pub alphabet: Vec<String>,
    pub cipher: Vec<String>,
}

impl EncrypterConfig {
    pub fn new() -> io::Result<Self> {
        // added extra random factor -> because each cipher was identifcal
        let mut rng = rand::thread_rng();
        let initial: u64 = rng.gen();

        // creating normal alphabet
        let default_alphabet = [ASCII_LETTERS, PUNCTUATION, DIGITS, SPACE].concat();
        let alphabet = default_alphabet
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        // creating cipher alphabet
        let mut rng = StepRng::new(initial, 13); // picking range
        let mut irs = Irs::default(); // using Inverse Riffle Shuffle to shuffle cipher alphabet

        // copying default for shuffling
        let mut key = default_alphabet
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let _shuffled = irs.shuffle(&mut key, &mut rng);

        Ok(Self {
            alphabet,
            cipher: key,
        })
    }

    // for future implementation: choosing your own alphabet
    // fn change_alphabet(&self, words: &[&str]) -> Vec<String> {
    //     let alphabet = words.concat();

    //     alphabet
    //         .chars()
    //         .map(|c| c.to_string())
    //         .collect::<Vec<String>>()

    // will then need to update cipher alphabet
    // }
}

pub fn encrypt_file(input_path: &str, econ: &EncrypterConfig) -> io::Result<(String, String)> {
    // check file exists - if it does, open it
    if !file_exists(input_path) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{}' not found.", input_path),
        ));
    }

    // encrypt the original file one chunk at a time
    // output the key path to: key_filename
    let (e_p, k_p) = process_file(input_path, econ)?;

    // output the new encrypted file path and key path
    println!("Paths are: {} and {}", e_p, k_p);
    Ok((e_p, k_p))
}

pub fn get_user_input(prompt: &str) -> io::Result<Vec<String>> {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();
    Ok(input
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>())
}

pub fn encrypt(input: &Vec<String>, econ: &EncrypterConfig) -> io::Result<String> {
    let mut cipher_text = String::new();

    for (letter) in input {
        if let Some(index) = econ.alphabet.iter().position(|x| x == letter) {
            // println!("letter {} is at index {} in original string", letter, index);
            cipher_text.push_str(&econ.cipher[index]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Letter '{}' not found in alphabet", letter),
            ));
        }
    }

    Ok(cipher_text)
}

pub fn decrypt(input: &String, econ: &EncrypterConfig) -> io::Result<String> {
    let mut plain_text = String::new();

    let cipher_iter = input
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    for letter in &cipher_iter {
        if let Some(index) = econ.cipher.iter().position(|x| x == letter) {
            plain_text.push_str(&econ.alphabet[index]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Letter '{}' not found in alphabet", letter),
            ));
        }
    }

    Ok(plain_text)
}

fn bytes_to_string(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .chars()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

pub fn file_exists(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}

fn output_key(file_path: &str, content: &Vec<String>) -> io::Result<String> {
    // Open or create a new file for writing
    let output_file_name = format!("key_{}", file_path);
    let mut file = File::create(&output_file_name)?;

    let content = content.join("");

    // Write the string to the file
    file.write_all(content.as_bytes())?;

    // Ensure all data is written to the file
    file.flush()?;
    Ok(output_file_name)
}

pub fn process_file(input_file_path: &str, econ: &EncrypterConfig) -> io::Result<(String, String)> {
    // Open input file for reading
    let input_file = File::open(input_file_path)?;
    let mut input_reader = BufReader::new(input_file);

    let output_file_path = format!("encrypted_{}", input_file_path);

    // Create or open output file for writing
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)?;
    let mut output_writer = BufWriter::new(output_file);

    // Create a buffer to store data read from input file
    let mut buffer = [0; 1024]; // You can adjust the buffer size as needed

    // Iterate over input file contents
    loop {
        let bytes_read = input_reader.read(&mut buffer)?;

        if bytes_read == 0 {
            // End of file
            break;
        }

        // Encrypt data read from buffer
        let new_data_to_encrypt = bytes_to_string(&buffer[..bytes_read]);
        let encrypted_data = encrypt(&new_data_to_encrypt, econ)?;
        // Write encrypted data to output file
        output_writer.write_all(encrypted_data.as_bytes())?;
    }

    // Flush buffered output to ensure all data is written
    output_writer.flush()?;

    let key_path = output_key(&input_file_path, &econ.cipher)?;

    Ok((output_file_path, key_path))
}
