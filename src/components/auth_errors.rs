use leptos::*;

#[component]
pub fn AuthErrors(cx: Scope, errors: Option<Vec<String>>) -> impl IntoView {
    view! { cx,
        <ul class="error-messages">
            {errors
                .into_iter()
                .map(|error| {
                    view! { cx, <li>{error}</li> }
                })
                .collect_view(cx)}
        </ul>
    }
}
