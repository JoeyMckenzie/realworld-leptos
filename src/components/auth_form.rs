use leptos::*;
use leptos_router::ActionForm;

use crate::{
    components::auth_errors::AuthErrors, models::users::AuthResponseContext,
    services::users::UsersService,
};

#[server(SubmitAuthForm, "/api")]
#[tracing::instrument(skip(password))]
pub async fn submit_auth_form(
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
) -> Result<AuthResponseContext, ServerFnError> {
    let service = UsersService::new();
    let response = service
        .register(
            username.unwrap_or_default(),
            email.unwrap_or_default(),
            password.unwrap_or_default(),
        )
        .await?;
    Ok(response)
}

#[component]
pub fn AuthForm(cx: Scope, #[prop(default = true)] include_username: bool) -> impl IntoView {
    let submit_auth_form = create_server_action::<SubmitAuthForm>(cx);
    let submit_auth_form_value = submit_auth_form.value();

    let button_text = move || {
        if include_username {
            "Sign up"
        } else {
            "Sign in"
        }
    };

    view! { cx,
        <AuthErrors errors=submit_auth_form_value/>
        <ActionForm action=submit_auth_form>
            <Show when=move || include_username fallback=|_| {}>
                <fieldset class="form-group">
                    <input
                        class="form-control form-control-lg"
                        type="text"
                        name="username"
                        placeholder="Your Name"
                    />
                </fieldset>
            </Show>
            <fieldset class="form-group">
                <input
                    class="form-control form-control-lg"
                    type="text"
                    name="email"
                    placeholder="Email"
                />
            </fieldset>
            <fieldset class="form-group">
                <input
                    class="form-control form-control-lg"
                    type="password"
                    name="password"
                    placeholder="Password"
                />
            </fieldset>
            <button class="btn btn-lg btn-primary pull-xs-right" type="submit">
                {button_text}
            </button>
        </ActionForm>
    }
}
