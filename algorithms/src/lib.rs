use std::string::String;
use utils::{hexstr_to_vec, read_lines};
use metrics::*;
use utils::*;


pub fn fixed_xor(hs1_bytes: Vec<u8>, hs2_bytes: Vec<u8>) -> Vec<u8> {
    hs1_bytes.iter()
        .zip(hs2_bytes.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn single_byte_xor(byte: u8, data: &Vec<u8>) -> Vec<u8> {
    data.iter().map(|b| byte ^ b).collect()
}

pub fn decrypt_single_byte_xor(ciphertext: Vec<u8>, score_fn: fn(&Vec<u8>) -> f64) -> (f64, u8, String){
    let mut min_score = 100000.0;
    let mut key = 0;
    let mut min_score_sentence = vec![];

    for i in 0..128 {
        let text = single_byte_xor(i, &ciphertext);
        let current_score = score_fn(&text);
        if min_score > current_score {
            key = i;
            min_score = current_score;
            min_score_sentence = text;
        }
    }

    if let Ok(text) = String::from_utf8(min_score_sentence) {
        (min_score, key, text)
    } else {
        (min_score, key, "".to_string())
    }
}

pub fn repeating_key_xor(key: Vec<u8>, text: Vec<u8>) -> Vec<u8> {
    let mut key_bytes_cycle_iterator = key.iter().cycle();

    let block_key = text.iter()
        .map(|_b| key_bytes_cycle_iterator.next())
        .flatten().cloned().collect();

    fixed_xor(block_key, text)
}

pub fn detect_single_byte_xor_from_file(filename: &str) -> (f64, u8, String, String) {
    let mut min_score = 1000000.0;
    let mut min_key= 0;
    let mut the_sentence = "".to_string();
    let mut the_ciphertext = "".to_string();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ciphertext) = line {
                let data = hexstr_to_vec(&ciphertext.clone());
                let (score, key, sentence) = decrypt_single_byte_xor(data, count_letters);
                if min_score > score {
                    min_score = score;
                    min_key = key;
                    the_sentence = sentence.trim().to_string();
                    the_ciphertext = ciphertext;
                }
            }
        }
    }
    (min_score, min_key, the_sentence, the_ciphertext)
}

// With help from https://www.megacolorboy.com/posts/the-cryptopals-crypto-challenges-set-1--break-repeatingkey-xor/
pub fn guess_keysize(ciphertext: &Vec<u8>) -> (usize, f64){
    let ciphertext_len = ciphertext.len();
    let mut min_score = 1000.0;
    let mut current_score = 0.0;
    let mut guessed_keysize = 0;
    for keysize in 2..40 {
        let mut running_means = vec![];
        for i in 0..(ciphertext_len / keysize - 1) {
            let c1 = &ciphertext[(i * keysize)..((i + 1) * keysize)];
            let c2 = &ciphertext[((i + 1) * keysize)..((i + 2) * keysize)];
            let distance = (hamming_distance(&c1.to_vec(), &c2.to_vec()) as f64) / (keysize as f64) ;
            running_means.push(distance);
        }
        current_score = running_means.iter().fold(0.0, |a, b| a + b) / (running_means.len() as f64);
        if current_score < min_score {
            min_score = current_score;
            guessed_keysize = keysize;
        }
    }
    (guessed_keysize, min_score)
}

pub fn decipher(ciphertext: &Vec<u8>, keysize: usize) -> Vec<u8> {
    let ciphertext_clone = ciphertext.clone();
    let mut key_bytes = vec![];
    for i in 0..keysize {
        let my_cipher: Vec<u8> = (0..ciphertext_clone.len())
        .filter(|x| x % keysize == i)
        .map(|i| ciphertext_clone[i])
        .collect();
        let (_, key, _) = decrypt_single_byte_xor(my_cipher, get_sentence_score);
        key_bytes.push(key);
    }
    repeating_key_xor(key_bytes, ciphertext.to_vec())
}

#[test]
fn fixed_xor_works() {
    let s1 = hexstr_to_vec("1c0111001f010100061a024b53535009181c");
    let s2 = hexstr_to_vec("686974207468652062756c6c277320657965");
    let expected = hexstr_to_vec("746865206b696420646f6e277420706c6179");
    let result = fixed_xor(s1, s2);
    assert_eq!(expected, result);
}

#[test]
fn repeating_key_xor_works() {
    let s1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
    let key = "ICE".as_bytes().to_vec();
    let result = repeating_key_xor(key, s1);
    let expected = hexstr_to_vec("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    assert_eq!(expected, result);
}