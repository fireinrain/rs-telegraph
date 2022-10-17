use crate::{error::*, types::*};
use std::{collections::HashMap, path::Path};

use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct AccountBuilder {
    short_name: String,
    access_token: Option<String>,
    author_name: Option<String>,
    author_url: Option<String>,
}

impl AccountBuilder {
    pub fn new(short_name: &str) -> Self {
        AccountBuilder {
            short_name: short_name.to_owned(),
            ..Default::default()
        }
    }

    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = short_name.to_owned();
        self
    }
}




