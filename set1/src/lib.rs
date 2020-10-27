use algorithms::*;
use metrics::*;
use utils::*;
use std::str;
use openssl::symm::{decrypt, Cipher};

pub fn challenge_1() {
    let hex_s = hexstr_to_vec("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let data = hex_to_base64(hex_s);
    println!("{:?}", str::from_utf8(&data).unwrap());
}

pub fn challenge_2() {
    let s1 = hexstr_to_vec("1c0111001f010100061a024b53535009181c");
    let s2 = hexstr_to_vec("686974207468652062756c6c277320657965");
    let data = fixed_xor(s1, s2);
    println!("{:?}", str::from_utf8(&data).unwrap());
}

pub fn challenge_3() {
    let encrypted_message = hexstr_to_vec("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let (score, key, sentence) = decrypt_single_byte_xor(encrypted_message, count_letters);
    println!("score: {}, key: {}, sentence: {:?}", score, key, sentence);
}

pub fn challenge_4() {
    let (score, key, sentence, ciphertext) = detect_single_byte_xor_from_file("data/4.txt");
    println!("score: {}, key: {}, sentence: {:?}, ciphertext: {:?}", score, key, sentence, ciphertext);
}

pub fn challenge_5() {
    let s1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();
    let key = "ICE".as_bytes().to_vec();
    let result = repeating_key_xor(key, s1);
    println!("{:?}", result);
}

pub fn challenge_6(){
    match get_ciphertext_from_file("data/6.txt") {
        Ok(ciphertext) => {
            let (keysize, _score) = guess_keysize(&ciphertext); 
            let data = decipher(&ciphertext, keysize);
            println!("{}", str::from_utf8(&data).unwrap());
        },
        _ => ()
    }
}

pub fn challenge_7(){
    let cipher = Cipher::aes_128_ecb();
    let key = "YELLOW SUBMARINE".as_bytes();
    match get_ciphertext_from_file("data/7.txt") {
        Ok(ciphertext) => {
            if let Ok(data) = decrypt(cipher, key, None, &ciphertext) {
                println!("{:?}", str::from_utf8(&data));
            }
        },
        Err(_err) => ()
    }
}