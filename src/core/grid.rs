use super::*;
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub enum GridError {
    AccessError,
}

impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GridError::AccessError => write!(f, "Could not access the collection"),
            _ => todo!(),
        }
    }
}

impl Error for GridError {}
