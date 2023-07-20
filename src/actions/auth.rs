use leptos::*;

use crate::services::users::UsersService;

#[server(SubmitAuthForm, "/api")]
#[tracing::instrument(skip(password))]
pub async fn submit_auth_form(
    username: Option<String>,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    let service = UsersService::new();
    let response = service.register(username.unwrap(), email, password).await?;
    Ok(())
}
