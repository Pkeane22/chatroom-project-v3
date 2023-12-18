use super::*;

#[component]
pub fn SignupPage() -> impl IntoView {
    let signup_user = create_server_action::<api::user::SignupUser>();

    view! {
        <div class="h-1/6"/>
        <div class=LOGIN_SIGNUP_CONTAINER_CLASS>
            <ActionForm action=signup_user>
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
                <label><b>{"Confirm Password:"}</b>
                    <input class=INPUT_CLASS
                        type="text"
                        name="confirm_password"
                        placeholder="Confirm Password"
                        autocomplete="new-password"
                        required
                    />
                </label>
                <ErrorComponent signal=signup_user.value()/>
                <button class=BUTTON_CLASS type="submit">"Sign Up"</button>
            </ActionForm>
            <div class="text-center text-sm">
                <p>"Already have an account?"</p>
                <A class="text-sky-500" href="/login">"Login"</A>
            </div>
        </div>
    }
}
