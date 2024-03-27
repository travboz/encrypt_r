const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const DIGITS: &str = "0123456789";
const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// use rand::Rng;
use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

fn main() {
    let mut rng = StepRng::new(2, 11);
    let mut irs = Irs::default();

    let options = format!("{}{}{} ", PUNCTUATION, DIGITS, ASCII_LETTERS);

    // let mut input = vec![1, 2, 3, 4, 5];

    let chars = options
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let mut key = options
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let input = "The quick brown fox jumps over the lazy dog"
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let mut cipher_text = String::new();

    let shuffled = irs.shuffle(&mut key, &mut rng);

    for letter in &input {
        if let Some(index) = chars.iter().position(|x| x == letter) {
            // println!("letter {} is at index {} in original string", letter, index);
            cipher_text.push_str(&key[index]);
        } else {
            println!("letter '{}' not found", letter);
        }
    }

    println!("Cipher text is now: {cipher_text}");

    // Decryption
    let mut plain_text = String::new();

    let cipher_iter = cipher_text
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    for letter in &cipher_iter {
        if let Some(index) = key.iter().position(|x| x == letter) {
            plain_text.push_str(&chars[index]);
        } else {
            println!("letter '{}' not found", letter);
        }
    }

    println!("Cipher text is now: {cipher_text}");
    println!("Original text is now: {plain_text}");
}
