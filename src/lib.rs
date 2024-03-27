// default alphabet
const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const DIGITS: &str = "0123456789";
const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPACE: &str = " ";

use rand::rngs::mock::StepRng;
use rand::Rng;

use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::io;

pub struct Encrypter {
    pub alphabet: Vec<String>,
    pub cipher: Vec<String>,
}

impl Encrypter {
    pub fn new() -> io::Result<Self> {
        // added extra random factor -> because each cipher was identifcal
        let mut rng = rand::thread_rng();
        let initial: u64 = rng.gen();

        // creating normal alphabet
        let default_alphabet = [ASCII_LETTERS, PUNCTUATION, DIGITS, SPACE].concat();
        println!("Len of default alphabet is: {}", default_alphabet.len());
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

    pub fn encrypt(&self, input: &Vec<String>) -> io::Result<String> {
        let mut cipher_text = String::new();

        for letter in input {
            if let Some(index) = self.alphabet.iter().position(|x| x == letter) {
                // println!("letter {} is at index {} in original string", letter, index);
                cipher_text.push_str(&self.cipher[index]);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Letter '{}' not found in alphabet", letter),
                ));
            }
        }

        Ok(cipher_text)
    }

    pub fn decrypt(&self, input: &String) -> io::Result<String> {
        let mut plain_text = String::new();

        let cipher_iter = input
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        for letter in &cipher_iter {
            if let Some(index) = self.cipher.iter().position(|x| x == letter) {
                plain_text.push_str(&self.alphabet[index]);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Letter '{}' not found in alphabet", letter),
                ));
            }
        }

        Ok(plain_text)
    }
}