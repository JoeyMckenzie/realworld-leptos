use leptos::{leptos_dom::console_log, tracing::info, *};
use leptos_router::ActionForm;

use crate::{models::users::User, services::users::UsersService};

#[server(SubmitAuthForm, "/api")]
#[tracing::instrument(skip(password))]
pub async fn submit_auth_form(
    username: Option<String>,
    email: String,
    password: String,
) -> Result<User, ServerFnError> {
    let service = UsersService::new();

    match service.register(username.unwrap(), email, password).await {
        Ok(user) => Ok(user),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }

    // if let Err(e) = response {
    //     match e {
    //         AppError::ValidationFailed(err) => {

    //         },
    //         _ => return Err(e.into()),
    //     }
    // }
}

#[component]
pub fn AuthForm(cx: Scope, #[prop(default = true)] include_username: bool) -> impl IntoView {
    let submit_auth_form = create_server_action::<SubmitAuthForm>(cx);

    // holds the latest *returned* value from the server
    let submit_auth_value = submit_auth_form.value();

    // check if the server has returned an error
    let has_error = move || {
        submit_auth_value.with(|val| {
            if let Some(Err(submit_auth_error)) = val {
                console_log(&format!("{}", submit_auth_error));
            }
            // matches!(val, Some(Err(_)));
        })
    };

    let button_text = move || {
        if include_username {
            "Sign up"
        } else {
            "Sign in"
        }
    };

    view! { cx,
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
            <button class="btn btn-lg btn-primary pull-xs-right" type="submit">{button_text}</button>
        </ActionForm>
    }
}
