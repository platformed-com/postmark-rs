//! You'll find in email sending related endpoints.

use std::fmt;

use serde::{Deserialize, Serialize};

pub use create_server::*;
pub use get_server::*;
pub use list_servers::*;

mod create_server;
mod get_server;
mod list_servers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerIdOrName {
    ServerId(isize),
    ServerName(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliveryType {
    #[default]
    Live,
    Sandbox,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerColor {
    #[default]
    Purple,
    Blue,
    Turquoise,
    Green,
    Red,
    Yellow,
    Grey,
    Orange,
}

impl fmt::Display for ServerIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerIdOrName::ServerId(id) => write!(f, "{}", id),
            ServerIdOrName::ServerName(name) => write!(f, "{}", name),
        }
    }
}
