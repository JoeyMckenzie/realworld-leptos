use crate::{
    components::{footer::Footer, navbar::Navbar},
    error_template::{AppError, ErrorTemplate},
    routes::{home::Home, login::Login, register::Register},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Meta charset="utf-8"/>
        <Meta name="description" content="A RealWorld implementation with Rust and Leptos."/>
        <Stylesheet id="leptos" href="/pkg/realworld-leptos.css"/>
        <Title text="Conduit"/>
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <main>
                <Navbar/>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <Home/> }
                        }
                    />
                    <Route
                        path="/register"
                        view=|cx| {
                            view! { cx, <Register/> }
                        }
                    />
                    <Route
                        path="/login"
                        view=|cx| {
                            view! { cx, <Login/> }
                        }
                    />
                </Routes>
                <Footer/>
            </main>
        </Router>
    }
}
