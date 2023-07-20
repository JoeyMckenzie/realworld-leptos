use leptos::*;
use leptos_router::A;

use crate::components::auth_errors::AuthErrors;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Sign in"</h1>
                        <p class="text-xs-center">
                            <A href="/register">"Need an account?"</A>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
