use std::collections::HashMap;

use serde::Deserialize;

pub type ErrorInfo = HashMap<String, Vec<String>>;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiError {
    pub errors: ErrorInfo,
}
