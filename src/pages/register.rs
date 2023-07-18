use leptos::*;
use leptos_router::A;

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
                        <ul class="error-messages">
                            <li>"That email is already taken"</li>
                        </ul>
                        <form>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Your Name"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button class="btn btn-lg btn-primary pull-xs-right">"Sign up"</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
