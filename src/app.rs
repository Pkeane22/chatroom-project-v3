use crate::api;
use leptos::*;
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
                    <Route path="/" view=HomePage/>
                    <Route path="/home" view=HomePage/>
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
    view! {<p>"Home"</p>}
}

#[component]
fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<api::user::LoginUser>();

    view! {
        <div class="container">
            <ActionForm action=login_user>
                <label><b>{"Enter Username:"}</b>
                    <input type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=login_user.value()/>
                <button type="submit">"Login"</button>
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
                <label><b>{"Enter Username:"}</b>
                    <input type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <label><b>{"Confirm Password:"}</b>
                    <input type="text"
                        name="confirm_password"
                        placeholder="Confirm Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=signup_user.value()/>
                <button type="submit">"Sign Up"</button>
            </ActionForm>
            <div class="a_container">
                <p>"Already have an account?"</p>
                <a href="/login">"Login"</a>
            </div>
        </div>
    }
}

#[component]
fn LoginSignupForm<T: Clone + ServerFn>(
    action: Action<T, Result<(), ServerFnError>>,
    is_login: bool,
) -> impl IntoView {
    view! {
            <ActionForm action>
                <label><b>{"Enter Username:"}</b>
                    <input type="text" name="username" placeholder="Username" autocomplete="username"/>
                </label>

                <label><b>{"Enter Password:"}</b>
                    <input type="text" name="password" placeholder="Password" autocomplete={if is_login {"current-password"} else {"new-password"}}/>
                </label>

    //            <Show when=move || { !is_login }>
    //                <label><b>{"Confirm Password:"}</b>
    //                    <input type="text" name="confirm_password" placeholder="Password" autocomplete="new-password"/>
    //                </label>
    //            </Show>

                <button type="submit">{if is_login {"Login"} else {"Sign Up"}}</button>
            </ActionForm>
        }
}

#[component]
fn ErrorComponent(signal: RwSignal<Option<Result<(), ServerFnError>>>) -> impl IntoView {
    {
        move || match signal.get() {
            Some(Err(ServerFnError::ServerError(error))) => {
                view! {<p style="color:red">{error}</p>}.into_view()
            }
            _ => view! {}.into_view(),
        }
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
