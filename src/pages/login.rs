use super::*;

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
                <a class="text-sky-500" href="/signup">"Sign Up"</a>
            </div>
        </div>
    }
}
