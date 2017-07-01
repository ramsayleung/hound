extern crate rand;
use rand::{thread_rng, Rng};
static BASE64_CHAR: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZa
bcdefghijklmnopqrstuvwxyz0123456789+/=";
static HEX_CHAR: &'static str = "1234567890abcdefABCDEF";
const THRESHOLD: u8 = 20;
fn main() {
    let mut random_strs: Vec<String> = vec![];
    for i in 0..10 {
        let length = thread_rng().gen_range(10, 30);
        let rstr: String = thread_rng()
            .gen_ascii_chars()
            .take(length)
            .collect::<String>();
        random_strs.push(rstr);
    }
    for rstr in random_strs.iter() {
        let valid_str = get_string_of_set(&rstr, &BASE64_CHAR);
        for v_word in valid_str.iter() {
            println!("shannon entropy: {}",
                     shannon_entropy(&v_word, &BASE64_CHAR));
            println!("valid word: {}", v_word);
        }
    }
}

fn shannon_entropy(data: &str, iterator: &str) -> f64 {
    let mut entropy = 0.0;
    for ch in iterator.chars() {
        let length = data.matches(ch).count() as f64;
        let p_x = length / data.len() as f64;
        if p_x > 0.0 {
            entropy += -p_x * p_x.log(2.0);
        }
    }
    entropy
}
fn get_string_of_set(word: &str, char_set: &str) -> Vec<String> {
    let mut count = 0;
    let mut letter: Vec<char> = vec![];
    let mut valid_words: Vec<String> = vec![];
    for ch in word.chars() {
        if char_set.matches(ch).count() > 0 {
            letter.push(ch);
            count += 1;
        } else {
            if count > THRESHOLD {
                valid_words.push(letter.iter().cloned().collect());
            }
            letter.clear();
            count = 0;

        }
    }
    if count > THRESHOLD {
        valid_words.push(letter.iter().clone().collect());
    }
    valid_words
}
