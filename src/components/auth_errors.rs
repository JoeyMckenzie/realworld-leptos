use leptos::*;

use crate::models::users::User;

#[component]
pub fn AuthErrors(
    cx: Scope,
    errors: RwSignal<Option<Result<User, ServerFnError>>>,
) -> impl IntoView {
    view! { cx,
        <ul class="error-messages">
            {move || {
                // let test = errors.get();
                // leptos::log!("{:?}", test);
                let errors: Option<Vec<String>> = if let Some(Err(context)) = errors.get() {
                    match context {
                        ServerFnError::ServerError(server_errors) => {
                            leptos::log!("{}", server_errors);
                            let parsed_errors: Vec<_> = server_errors
                                .split('|')
                                .map(|token| token.to_string())
                                .collect();
                            Some(parsed_errors)
                        }
                        _ => None,
                    }
                } else {
                    None
                };
                view! { cx,
                    {errors
                        .unwrap_or_default()
                        .into_iter()
                        .map(|error| {
                            view! { cx, <li>{error}</li> }
                        })
                        .collect_view(cx)}
                }
            }}
        </ul>
    }
}
