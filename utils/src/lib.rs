use hex::FromHex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use base64;


// This is challenge 1 from set 1
pub fn hex_to_base64(hex_string: Vec<u8>) -> Vec<u8> {
    base64::encode(hex_string).as_bytes().to_vec()
}

pub fn str_to_vec(data: &str) -> Vec<u8> {
    data.as_bytes().to_vec()
}

pub fn hexstr_to_vec(data: &str) -> Vec<u8> {
    Vec::from_hex(data).expect("Invalid parameter")
}

pub fn hamming_distance(b1: &Vec<u8>, b2: &Vec<u8>) -> u64 {
    b1.iter().zip(b2).fold(0, |a, (b, c)| a + (*b ^ *c).count_ones() as u64)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_ciphertext_from_file(filename: &str) -> std::result::Result<std::vec::Vec<u8>, base64::DecodeError>{
    match read_lines(filename) {
        Ok(lines) => base64::decode(lines.fold("".to_owned(), |a, b| [a, b.unwrap()].join(""))),
        Err(_) => Ok(vec![])
    }
}


#[test]
fn hex_to_base64_works() {
    let hex_s = hexstr_to_vec("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected = str_to_vec("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    let actual = hex_to_base64(hex_s);
    assert_eq!(expected, actual);
}

#[test]
fn test_hamming_distance() {
    let s1 = str_to_vec("this is a test");
    let s2 = str_to_vec("wokka wokka!!!");
    assert_eq!(37, hamming_distance(&s1, &s2))
}