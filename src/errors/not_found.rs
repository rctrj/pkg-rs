use proc_macros_rs::DisplayUsingDebug;

use super::{BaseError, MetaData, Resource};
use crate::delegate_params;

#[derive(Debug, DisplayUsingDebug)]
pub struct NotFoundError {
    pub base: BaseError,
    pub resource: Resource,
}

delegate_params!(NotFoundError);

impl NotFoundError {
    pub fn new(resource: Resource) -> Self {
        let base = BaseError::new();
        Self { base, resource }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.base = self.base.with_meta(MetaData::Id(id));
        self
    }
}
