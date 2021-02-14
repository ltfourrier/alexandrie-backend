use super::{init_app_state, AppState};
use crate::cfg::read_configuration;

use once_cell::sync::Lazy;
use std::sync::Mutex;

mod health;
mod users;

/// Create an application state that is suitable for tests.
///
/// Using this method instead of constructing an application state for each test is recommended, as
/// it will share some resources (like the database connection pool) between different application
/// states.
pub async fn app_state() -> AppState {
    static APP_STATE: Lazy<Mutex<Option<AppState>>> = Lazy::new(|| Mutex::new(None));

    let mut app_state = APP_STATE.lock().unwrap();

    if app_state.is_none() {
        let cfg = read_configuration().unwrap();
        *app_state = Some(init_app_state(&cfg).await.unwrap());
    }

    app_state.as_ref().unwrap().clone()
}
