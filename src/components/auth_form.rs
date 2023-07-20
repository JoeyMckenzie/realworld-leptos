use leptos::*;
use leptos_router::ActionForm;

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

#[component]
pub fn AuthForm(cx: Scope, #[prop(default = true)] include_username: bool) -> impl IntoView {
    let submit_auth_form = create_server_action::<SubmitAuthForm>(cx);
    // holds the latest *returned* value from the server
    let auth_result = submit_auth_form.value();
    // check if the server has returned an error
    let has_error = move || {
        auth_result.with(|val| {
            let test = val.as_ref().unwrap();
            let testt = test.as_ref().unwrap_err();
            matches!(val, Some(Err(_)));
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
