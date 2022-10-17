use serde::Deserialize;
use crate::error::Error;


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    OK { result: T },
    Err { ok: bool, error: String },
}

impl <T> Into<Result<T,Error>> for ApiResult<T>{
    fn into(self) -> Result<T, Error> {
        match self {
            ApiResult::OK { result: v } => Ok(v),
            ApiResult::Err { error: e, .. } => Err(Error::ApiError(e)),
        }
    }
}