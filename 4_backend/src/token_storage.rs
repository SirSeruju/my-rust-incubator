use ring::rand as rrand;
use std::collections::HashMap;
use std::sync::Mutex;

/// Allows store and get tokens
pub trait TokenStorage<U, T> {
    /// Get new token by user
    fn new_token(&self, user: U) -> T;
    /// Returns user by token, if token exist
    fn validate(&self, token: &T) -> Option<U>;
}

/// Type for token
type Token = String;
/// Type for username
type Username = String;

pub struct Storage {
    tokens: Mutex<HashMap<Token, Username>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            tokens: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns cryptographically secure with length = 64
fn new_access_token() -> Token {
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
    token.iter().collect::<Token>()
}

impl TokenStorage<Token, Username> for Storage {
    fn new_token(&self, user: Username) -> Token {
        let token = new_access_token();
        self.tokens.lock().unwrap().insert(token.clone(), user);
        token
    }

    fn validate(&self, token: &Token) -> Option<Username> {
        self.tokens.lock().unwrap().get(token).cloned()
    }
}
