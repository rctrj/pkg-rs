use std::error::Error as StdErr;

use super::Error;

pub fn map_err<T: StdErr + 'static>(err: T) -> Error { Error::Unhandled(Box::new(err)) }

macro_rules! map_err {
    ($feature: expr, $name: ident, $err: path) => {
        #[cfg(feature = $feature)]
        pub mod $name {
            map_err!($err);
        }
    };

    ($err: path) => {
        impl From<$err> for super::Error {
            fn from(value: $err) -> Self { super::map_err(value) }
        }
    };
}

map_err!("config", config, config::ConfigError);
map_err!("fast_uaparser", fast_uaparser, fast_uaparser::ParserError);
map_err!("jwt", jwt, jwt::Error);
map_err!("mongodb", mongodb, mongodb::error::Error);
map_err!("phonenumber", phonenumber, phonenumber::ParseError);

#[cfg(feature = "diesel")]
pub mod diesel {
    map_err!(diesel::result::Error);
    map_err!(diesel::result::ConnectionError);
    map_err!(diesel::r2d2::Error);
}

#[cfg(feature = "deadpool")]
pub mod deadpool {
    use deadpool::managed::PoolError;

    impl<T> From<PoolError<T>> for super::Error
    where
        T: std::error::Error + 'static,
    {
        fn from(value: PoolError<T>) -> Self { super::map_err(value) }
    }
}
