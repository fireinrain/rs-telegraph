use crate::{error::*, types::*};
use std::{collections::HashMap, path::Path};

use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct AccountBuilder {
    access_token: Option<String>,
    short_name: String,
    author_name: Option<String>,
    author_url: Option<String>,
}

impl AccountBuilder{

}




