use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type ErrorInfo = HashMap<String, Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub errors: ErrorInfo,
}

impl ApiError {
    pub fn into_errors(self) -> Vec<String> {
        self.errors
            .into_iter()
            .flat_map(|(error_key, error_values)| {
                error_values
                    .into_iter()
                    .map(move |error_value| format!("{} {}", error_key, error_value))
                    .collect::<Vec<String>>()
            })
            .collect::<_>()
    }
}
