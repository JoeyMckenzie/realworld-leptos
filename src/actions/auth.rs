#![allow(unused_imports)]

use leptos::*;

use crate::{error_template::AppError, models::users::User, services::users::UsersService};

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
            let user_cookie = format!(
                "conduit_user={}; SameSite=None; Secure; Max-Age=3600",
                serialized_user
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
) -> Result<(), ServerFnError> {
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
            let user_cookie = format!("conduit_user={}", serialized_user);
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

            Ok(())
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
