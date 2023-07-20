use leptos::*;

#[component]
pub fn AuthErrors(cx: Scope, errors: ReadSignal<Vec<String>>) -> impl IntoView {
    view! { cx,
        <Show
            when=move || !errors.get().is_empty()
            fallback=|_| {
                view! { cx,  }
            }
        >
            <ul class="error-messages">
                {errors
                    .get()
                    .into_iter()
                    .map(|error| {
                        view! { cx, <li>{error}</li> }
                    })
                    .collect_view(cx)}
            </ul>
        </Show>
    }
}
