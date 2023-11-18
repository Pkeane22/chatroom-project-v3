use crate::api;
use leptos::{*, html::Input};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Chatroom"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/signup" view=SignupPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {}
}

#[component]
fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<api::user::LoginUser>();

    view! {
        <div class="container">
            <ActionForm action=login_user>
                <label><b>{"Enter Username:"}</b></label>
                <input type="text" name="username" placeholder="Username"/>

                <label><b>{"Enter Password:"}</b></label>
                <input type="text" name="password" placeholder="Password"/>

                <button type="submit">{"Log In"}</button>
            </ActionForm>

            <div class="a_container">
                <p>"Don't have an account?"</p>
                <a href="/signup">"Sign Up"</a>
            </div>
        </div>

    }
}

#[component]
fn SignupPage() -> impl IntoView {
    let signup_user = create_server_action::<api::user::SignupUser>();

    view! {
        <div class="container">
            <ActionForm action=signup_user>
                <label><b>{"Enter Username:"}</b></label>
                <input type="text" name="username" placeholder="Username"/>

                <label><b>{"Enter Password:"}</b></label>
                <input type="text" name="password" placeholder="Password"/>

                <button type="submit">{"Sign Up"}</button>
            </ActionForm>

            <div class="a_container">
                <p>"Already have an account?"</p>
                <a href="/login">"Login"</a>
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
