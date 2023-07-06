use anyhow::Result;
use phonenumber::{Mode, PhoneNumber};

#[derive(Debug, Clone)]
pub struct Phone {
    inner: PhoneNumber,
}

impl Phone {
    pub fn new(number: &str) -> Result<Self> {
        let inner = phonenumber::parse(None, number)?;
        Ok(Self { inner })
    }
}

impl ToString for Phone {
    fn to_string(&self) -> String { self.inner.format().mode(Mode::E164).to_string() }
}
