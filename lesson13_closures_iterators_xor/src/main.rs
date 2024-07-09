use std::{cmp::Ordering, io};

use bytes::Bytes;

fn main() {
    println!("Enter string");
    let sipher_data: Vec<u8> = sipher();
    println!("{sipher_data:?}");
    println!("Enter your cipher data");
    let desipher_data: String = desipher();
    println!("{desipher_data:?}")
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().into()
}

fn sipher() -> Vec<u8> {
    let input = read_str();
    println!("Enter pwd");
    let pwd = pad_or_trim(input.len(), read_str().as_bytes());
    zip_and_xor(&pwd, Bytes::from(input))
}
fn desipher() -> String {
    let input: Vec<u8> = read_str()
        .split(",")
        .map(|el| el.trim().parse::<u8>().expect("InvalidInput"))
        .collect();
    println!("Enter pwd");
    let pwd = pad_or_trim(input.len(), read_str().as_bytes());
    let back = zip_and_xor(&pwd, Bytes::from(input));
    String::from_utf8(back).unwrap()
}

fn zip_and_xor(pwd: &[u8], bytes_data: Bytes) -> Vec<u8> {
    pwd.iter()
        .zip(bytes_data) // [1,2,3] + [2,3,4] => [(1,2), (2,3), (3,4)]
        .map(|(&a, b)| a ^ b)
        .collect()
}

fn pad_or_trim(limit: usize, original_pass: &[u8]) -> Vec<u8> {
    let pwd_len = original_pass.len();

    match limit.cmp(&pwd_len) {
        Ordering::Less => original_pass.iter().copied().take(limit).collect(),
        Ordering::Greater => {
            let mut as_bytes: Vec<u8> = original_pass.to_vec();
            for i in pwd_len..limit {
                as_bytes.push(as_bytes[i % pwd_len]);
            }
            as_bytes
        }
        Ordering::Equal => original_pass.to_vec(),
    }
}
