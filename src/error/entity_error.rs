use std::{error, fmt};
use std::fmt::{Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{EntityError::*, WalletError};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum EntityError {
    WalletOperationError {error : WalletError},
}

impl error::Error for EntityError {}

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            WalletOperationError {error} =>
                write!(f, "A wallet operation generated the following error -> {}", error),
        }
    }
}