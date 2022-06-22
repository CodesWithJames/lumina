use castle_api::types::State;

use super::User;

pub struct Root;

#[castle_api::castle_macro(Type)]
impl Root {
    fn ping(&self, _state: &State) -> String {
        "pong".to_string()
    }

    async fn create_user(
        &self,
        state: &State,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<(), anyhow::Error> {
        User::create_user(state, email, password, first_name, last_name)
            .await
            .map_err(|e| e.into())
    }

    async fn login(
        &self,
        state: &State,
        email: &str,
        password: &str,
    ) -> Result<String, anyhow::Error> {
        User::login(state, email, password)
            .await
            .map_err(|e| e.into())
    }
}
