use utils::str_to_vec;
use std::collections::HashMap;

// chi square score, based on english plaintext character frequencies.
//
// if the score == 1000, it means (for now) that the array has some
// impossible characters, since we want this socre as low as possible
pub fn get_sentence_score(data: &Vec<u8>) -> f64 {
    let frequencies: [f64; 26] = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06094,
        0.00153, 0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987,
        0.06327, 0.09056, 0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074 ];
    let mut count: [u8; 26] = [0; 26];
    let mut ignored = 0;
    let mut score = 0.0;
    for b in data {
        if 65 <= *b && *b <= 90 { count[(*b - 65) as usize] += 1; }
        else if 97 <= *b && *b <= 122 { count[(*b - 97) as usize] += 1; }
        else if 32 <= *b && *b <= 126 { ignored += 1; }
        else if *b == 9 || *b == 10 || *b == 13 { ignored += 1; }
        else { return 100000.0; }
    }
    let len = data.len() - ignored;
    for i in 0..26 {
        let expected = (len as f64) * frequencies[i];
        let d = (count[i] as f64) - expected;
        score += (d * d) / expected;
    }
    return score;
}

pub fn get_english_score(data: &Vec<u8>) -> f64 {
    let mut char_map = HashMap::new();
    char_map.insert('a', 0.08167);
    char_map.insert('b', 0.01492);
    char_map.insert('c', 0.02782);
    char_map.insert('d', 0.04253);
    char_map.insert('e', 0.12702);
    char_map.insert('f', 0.02228);
    char_map.insert('g', 0.02015);
    char_map.insert('h', 0.06094);
    char_map.insert('i', 0.06094);
    char_map.insert('j', 0.00153);
    char_map.insert('k', 0.00772);
    char_map.insert('l', 0.04025);
    char_map.insert('m', 0.02406);
    char_map.insert('n', 0.06749);
    char_map.insert('o', 0.07507);
    char_map.insert('p', 0.01929);
    char_map.insert('q', 0.00095);
    char_map.insert('r', 0.05987);
    char_map.insert('s', 0.06327);
    char_map.insert('t', 0.09056);
    char_map.insert('u', 0.02758);
    char_map.insert('v', 0.00978);
    char_map.insert('w', 0.02360);
    char_map.insert('x', 0.00150);
    char_map.insert('y', 0.01974);
    char_map.insert('z', 0.00074);
    char_map.insert(' ', 0.13000);
    let mut total: f64 = 0.0;
    for d in data {
        match char_map.get(&(*d as char)) {
            Some(val) => total += val,
            _ => ()
        }
    }
    total
}

pub fn count_letters(data: &Vec<u8>) -> f64 {
    let mut total = 0.0;
    for byte in data {
        if (*byte >= 65u8 && *byte < 122u8) || *byte == 32u8 {
            total = total - 1.0;
        }
    }
    total
}

#[test]
fn get_sentence_score_works() {
    let data = str_to_vec("this is SOOOOO wrong... I like it!");
    let total_score = get_sentence_score(&data);
    assert_eq!(total_score, 33.22341404416086);
}

#[test]
fn get_english_score_works() {
    let data = str_to_vec("this is SOOOOO wrong... I like it!");
    let total_score = get_english_score(&data);
    assert_eq!(total_score, 1.8135299999999992);
}

#[test]
fn count_letters_score_works() {
    let data = str_to_vec("this is SOOOOO wrong... I like it!");
    let total_score = count_letters(&data);
    assert_eq!(total_score, -30.0);
}