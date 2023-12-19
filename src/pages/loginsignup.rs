use super::*;

#[component]
pub fn LoginSignupPage<T: 'static + ServerFn + Clone>(
    is_signup: bool,
    action: Action<T, Result<String, ServerFnError>>,
) -> impl IntoView {
    create_effect(move |_| {
        let set_username = expect_context::<WriteSignal<Option<String>>>();
        let value = action.value().get();
        if let Some(Ok(username)) = value {
            log::debug!("Setting username to {}", username);
            set_username.set(Some(username));
            leptos_router::use_navigate()("/home", Default::default());
        }

    });

    view! {
        <div class="h-1/6"/>
        <div class=LOGIN_SIGNUP_CONTAINER_CLASS>
            <ActionForm  action=action>
                <label><b>{"Enter Username:"}</b>
                    <input class=INPUT_CLASS
                        type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input class=INPUT_CLASS
                        type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                {move || if is_signup {
                    view! {
                        <label><b>{"Confirm Password:"}</b>
                            <input class=INPUT_CLASS
                                type="text"
                                name="password"
                                placeholder="Password"
                                autocomplete="new-password"
                                required
                            />
                        </label>
                    }.into_view()
                } else {
                        view! {}.into_view()
                }}
                <ErrorComponent signal=action.value()/>
                <button class=BUTTON_CLASS type="submit">{if is_signup {"Signup"}else{"Login"}}</button>
            </ActionForm>
            <div class="text-center text-sm">
            {move || if is_signup {
                view! {
                    <p>"Already have an account?"</p>
                    <A class="text-sky-500" href="/login">"Login In"</A>
                }
            } else {
                view! {
                    <p>"Don't have an account?"</p>
                    <A class="text-sky-500" href="/signup">"Sign Up"</A>
                }
            }}
            </div>
        </div>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<api::user::LoginUser>();
    
    view! {
        <div class="h-1/6"/>
        <div class=LOGIN_SIGNUP_CONTAINER_CLASS>
            <ActionForm action=login_user>
                <label><b>{"Enter Username:"}</b>
                    <input class=INPUT_CLASS
                        type="text"
                        name="username"
                        placeholder="Username"
                        autocomplete="username"
                        required
                    />
                </label>
                <label><b>{"Enter Password:"}</b>
                    <input class=INPUT_CLASS
                        type="text"
                        name="password"
                        placeholder="Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=login_user.value()/>
                <button class=BUTTON_CLASS type="submit">"Login"</button>
            </ActionForm>
            <div class="text-center text-sm">
                <p>"Don't have an account?"</p>
                <A class="text-sky-500" href="/signup">"Sign Up"</A>
            </div>
        </div>
    }
}
