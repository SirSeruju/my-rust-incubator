use rand::seq::SliceRandom;
/// Generates random password with `length` and choosen symbols from `symbol_set`
fn generate_password(symbol_set: &str, length: usize) -> String {
    symbol_set
        .chars()
        .collect::<Vec<_>>()
        .choose_multiple(&mut rand::thread_rng(), length)
        .collect::<String>()
}

/// Select random value from `slice`
fn select_rand_val<T>(slice: &[T]) -> Option<&T> {
    slice.choose(&mut rand::thread_rng())
}

use ring::rand as rrand;
/// Returns cryptographically secure with length = 64
fn new_access_token() -> String {
    let sys_random = rrand::SystemRandom::new();
    let mut token = Vec::with_capacity(64);
    let symbol_set = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .chain(b'0'..=b'9')
        .map(|x| x as char)
        .collect::<Vec<char>>();
    for _ in 0..64 {
        let i: rrand::Random<[u8; 1]> = rrand::generate(&sys_random).unwrap();
        let i = i.expose()[0];
        let i: usize = i as usize % symbol_set.len();
        token.push(symbol_set[i]);
    }
    token.iter().collect::<String>()
}

use base64;
use sha3::{Digest, Sha3_256};
use std::fs;
/// Returs Sha3_256 hash of file for given `filepath`
fn get_file_hash(filepath: &str) -> String {
    let mut hasher = Sha3_256::new();
    // TODO: read file by chunks
    hasher.update(fs::read(filepath).unwrap());
    base64::encode(hasher.finalize())
}

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
/// Returs argon2 hash for given password
fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn main() {
    println!(
        r#"generate_password("abcde", 8) = {:?}"#,
        generate_password("abcde", 8)
    );
    println!(
        r#"select_rand_val(&[0, 1, 2, 3]) = {:?}"#,
        select_rand_val(&[0, 1, 2, 3])
    );
    println!(r#"new_access_token() = {:?}"#, new_access_token());
    println!(
        r#"get_file_hash("Cargo.toml") = {:?}"#,
        get_file_hash("Cargo.toml")
    );
    println!(
        r#"hash_password("Some password") = {:?}"#,
        hash_password("Some password")
    );
}
