//! You'll find in templates sending related endpoints.

use serde::{Deserialize, Serialize};
use std::fmt;

mod delete_suppression;
mod edit_message_stream;
mod get_suppressions;

pub use delete_suppression::*;
pub use edit_message_stream::*;
pub use get_suppressions::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SuppressionStatusType {
    #[default]
    Deleted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamIdOrName {
    StreamId(String),
}

impl fmt::Display for StreamIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamIdOrName::StreamId(id) => write!(f, "{}", id),
        }
    }
}
