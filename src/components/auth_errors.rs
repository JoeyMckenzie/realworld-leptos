use leptos::*;

use crate::models::users::AuthResponseContext;

#[component]
pub fn AuthErrors(
    cx: Scope,
    errors: RwSignal<Option<Result<AuthResponseContext, ServerFnError>>>,
) -> impl IntoView {
    view! { cx,
        <ul class="error-messages">
            {move || {
                let errors = if let Some(Ok(context)) = errors.get() {
                    match context {
                        AuthResponseContext::AuthenticatedUser(_) => None,
                        AuthResponseContext::ValidationError(validation_errors) => {
                            Some(validation_errors.into_errors())
                        }
                    }
                } else {
                    None
                };

                // TODO: this could most likely be cleaned up, somehow need to avoid the allocation to a default vec
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
