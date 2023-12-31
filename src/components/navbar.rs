use leptos::*;
use leptos_router::A;

use crate::actions::auth::GetUserFromCookie;

#[component]
fn UnauthenticatedNavbarItems(cx: Scope) -> impl IntoView {
    view! { cx,
        <li class="nav-item">
            <A class="nav-link" active_class="active" href="/login">
                "Sign in"
            </A>
        </li>
        <li class="nav-item">
            <A class="nav-link" active_class="active" href="/register">
                "Sign up"
            </A>
        </li>
    }
}

#[component]
fn AuthenticatedNavbarItems(cx: Scope, _username: String) -> impl IntoView {
    view! { cx,
        <li class="nav-item">
            <A class="nav-link" active_class="active" href="/articles/new">
                <i class="ion-compose"></i>
                " New Article"
            </A>
        </li>
        <li class="nav-item">
            <A class="nav-link" active_class="active" href="/settings">
                <i class="ion-gear-a"></i>
                " Settings"
            </A>
        </li>
        <li class="nav-item">
            <A class="nav-link" active_class="active" href="/profile">
                <img src="" class="user-pic"/>
                "Eric Simons"
            </A>
        </li>
    }
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let user_from_cookie = create_server_action::<GetUserFromCookie>(cx);
    let user_from_cookie_value = user_from_cookie.value();

    view! { cx,
        <nav class="navbar navbar-light">
            <div class="container">
                <A class="navbar-brand" href="/">
                    "conduit"
                </A>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        <A class="nav-link" active_class="active" href="/" exact=true>
                            "Home"
                        </A>
                    </li>
                    <Show
                        when=move || user_from_cookie_value.get().is_some()
                        fallback=|cx| {
                            view! { cx, <UnauthenticatedNavbarItems/> }
                        }
                    >
                        <AuthenticatedNavbarItems username=user_from_cookie_value.get().unwrap().unwrap().unwrap()/>
                    </Show>
                </ul>
            </div>
        </nav>
    }
}
