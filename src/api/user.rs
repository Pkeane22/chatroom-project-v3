use leptos::*;

#[server[LoginUser, "/api/login/login_user"]]
pub async fn login_user(username: String, password: String) -> Result<(), ServerFnError> {
    todo!()
}

#[server[SignupUser, "/api/login/signup_user"]]
pub async fn signup_user(username: String, password: String) -> Result<(), ServerFnError> {
    todo!()
}
