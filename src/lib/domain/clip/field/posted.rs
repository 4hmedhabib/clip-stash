use crate::domain::time::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct Posted(Option<Time>);

impl Posted {
    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}
