#![allow(clippy::crate_in_macro_def)]

use std::{error::Error as StdErr, fmt::Debug};

pub use already_exists::*;
pub use conv::*;
pub use not_found::*;
use proc_macros_rs::DisplayUsingDebug;

mod already_exists;
mod conv;
mod not_found;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, DisplayUsingDebug)]
#[non_exhaustive]
pub enum Error {
    NotFound(NotFoundError),
    AlreadyExists(AlreadyExistsError),
    Malformed(String),

    Unhandled(Box<dyn StdErr>),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MetaData {
    Id(i32),
    Resource(Resource),
    Meta(String),
    UserId(i32),
    DeviceId(String),
    App(String),
    DocumentId(String),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorParams {
    UserId(i32),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Resource {
    User,
    UserDevice,
    UserAgent,
    Topic,
    TopicChoice,
    TopicChoiceOpinion,
}

#[derive(Debug)]
pub struct BaseError {
    pub params: Vec<ErrorParams>,
    pub meta: Vec<MetaData>,
}

impl BaseError {
    fn new() -> Self {
        let params = Vec::new();
        let meta = Vec::new();
        Self { params, meta }
    }

    fn with_meta(mut self, meta: MetaData) -> Self {
        self.meta.push(meta);
        self
    }

    fn with_metas(mut self, metas: Vec<MetaData>) -> Self {
        for meta in metas {
            self = self.with_meta(meta);
        }

        self
    }

    fn with_param(mut self, param: ErrorParams) -> Self {
        self.params.push(param);
        self
    }

    fn with_params(mut self, params: Vec<ErrorParams>) -> Self {
        for param in params {
            self = self.with_param(param)
        }

        self
    }
}

#[macro_export]
macro_rules! delegate_params {
    ($err: ident) => {
        impl $err {
            pub fn with_meta(mut self, meta: super::MetaData) -> Self {
                self.base = self.base.with_meta(meta);
                self
            }

            pub fn with_metas(mut self, metas: Vec<super::MetaData>) -> Self {
                self.base = self.base.with_metas(metas);
                self
            }

            pub fn with_param(mut self, param: super::ErrorParams) -> Self {
                self.base = self.base.with_param(param);
                self
            }

            pub fn with_params(mut self, params: Vec<super::ErrorParams>) -> Self {
                self.base = self.base.with_params(params);
                self
            }
        }
    };
}

impl StdErr for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}
