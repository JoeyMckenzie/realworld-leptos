#![allow(unused_imports)]

use std::{cell::RefCell, rc::Rc};

use base64::{engine::general_purpose, Engine as _};
use leptos::*;
use serde::Deserialize;

use crate::{error_template::AppError, models::users::User, services::users::UsersService};

const USER_COOKIE_ID: &str = "conduit_user_token";

#[server(SubmitAuthForm, "/api")]
pub async fn submit_auth_form(
    cx: Scope,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
) -> Result<User, ServerFnError> {
    use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos_axum::ResponseOptions;

    let service = UsersService::new();
    let parsed_username = username.unwrap_or_default();
    leptos::log!("{parsed_username:?}");
    let response = if !parsed_username.is_empty() {
        service
            .register(
                parsed_username,
                email.unwrap_or_default(),
                password.unwrap_or_default(),
            )
            .await
    } else {
        service
            .login(email.unwrap_or_default(), password.unwrap_or_default())
            .await
    };

    match response {
        Ok(user) => {
            // create a cookie with the serialized user returned from the auth response
            let serialized_user = user.ser().unwrap();
            let user_bytes = serialized_user.as_bytes();
            let encoded_user = general_purpose::STANDARD_NO_PAD.encode(user_bytes);
            let user_cookie = format!(
                "{}={}; SameSite=None; Secure; Max-Age=3600",
                USER_COOKIE_ID, encoded_user
            );
            let mut response_headers = HeaderMap::new();
            response_headers.insert(SET_COOKIE, HeaderValue::from_str(&user_cookie).unwrap());

            // set some response parts for axum
            let response_parts = leptos_axum::ResponseParts {
                headers: response_headers,
                status: Some(StatusCode::OK),
            };

            // override the options the server function will return to the UI
            let response_options_outer = use_context::<leptos_axum::ResponseOptions>(cx);
            if let Some(response_options) = response_options_outer {
                response_options.overwrite(response_parts);
            }

            leptos_axum::redirect(cx, "/");

            Ok(user)
        }
        Err(app_error) => match app_error {
            AppError::ValidationFailed(validation_errors) => {
                let friendly_errors = validation_errors.into_errors().join("|");
                Err(ServerFnError::ServerError(friendly_errors))
            }
            AppError::ReqwestError(e) => Err(ServerFnError::ServerError(e.to_string())),
            _ => Err(ServerFnError::ServerError(
                "unexpected error occurred".into(),
            )),
        },
    }
}

#[server(GetUserFromCookie, "/api")]
pub async fn get_user_from_cookie(
    cx: Scope,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
) -> Result<Option<String>, ServerFnError> {
    use axum::{extract::Query, http::Method};
    use axum_extra::extract::cookie::{Cookie, CookieJar};
    use leptos_axum::extract;

    extract(cx, |jar: CookieJar| async move {
        leptos::log!("here 3");
        if let Some(user_session) = jar.get(USER_COOKIE_ID) {
            leptos::log!("here 2");
            let decoded_user = general_purpose::STANDARD_NO_PAD
                .decode(user_session.value())
                .unwrap();
            let user_str = std::str::from_utf8(&decoded_user).unwrap();
            let serialized_user: User = serde_json::from_str(user_str).unwrap();
            Some(serialized_user.username)
        } else {
            leptos::log!("here");
            None
        }
    })
    .await
    .map_err(|e| ServerFnError::ServerError("error while extracting request cookies".to_string()))
}
