use leptos::*;
use leptos_router::A;

use crate::components::{auth_errors::AuthErrors, auth_form::AuthForm};

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">"Sign up"</h1>
                        <p class="text-xs-center">
                            <A href="/login">"Have an account?"</A>
                        </p>
                        <AuthErrors/>
                        <AuthForm/>
                    </div>
                </div>
            </div>
        </div>
    }
}
