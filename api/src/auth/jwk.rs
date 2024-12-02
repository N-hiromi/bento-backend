use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwk {
    pub alg: String,
    pub e: String,
    kid: String,
    kty: String,
    pub n: String,
    r#use: String,
}

impl Jwks {
    pub fn find(&self, kid: &str) -> Option<Jwk> {
        self.keys.iter().find(|key| key.kid == kid).cloned()
    }
}
