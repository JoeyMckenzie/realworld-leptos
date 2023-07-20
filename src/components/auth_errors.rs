use leptos::*;

#[component]
pub fn AuthErrors(cx: Scope) -> impl IntoView {
    view! { cx,
        <ul class="error-messages">
            <li>"That email is already taken"</li>
        </ul>
    }
}
