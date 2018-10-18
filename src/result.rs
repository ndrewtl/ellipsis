use std::result;
use ::Error;

pub type Result<T> = result::Result<T, Error>;
