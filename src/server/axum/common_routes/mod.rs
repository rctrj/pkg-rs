mod graphql;

pub use graphql::*;

pub async fn ping<'a>() -> &'a str { "pong" }
