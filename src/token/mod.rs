use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{FromBase64, Header, SignWithKey, Token, VerifyWithKey};
use serde::Serialize;
use sha2::Sha256;

#[derive(Clone)]
pub struct JwtToken {
    signing_key: Hmac<Sha256>,
}

impl JwtToken {
    pub fn new(signing_key: &str) -> Result<Self> {
        let signing_key: Hmac<Sha256> = Hmac::new_from_slice(signing_key.as_ref())?;
        Ok(Self { signing_key })
    }

    pub fn generate<T: Serialize>(&self, claims: T) -> Result<String> {
        let header = Header::default();
        let token = Token::new(header, claims);

        let token = token.sign_with_key(&self.signing_key)?;
        Ok(token.into())
    }

    pub fn validate<T: FromBase64>(&self, token: &str) -> Result<T> {
        let token: Token<Header, T, _> = Token::parse_unverified(token)?;
        let token = token.verify_with_key(&self.signing_key)?;
        let (_, claims) = token.into();
        Ok(claims)
    }
}
