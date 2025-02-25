use std::sync::{Mutex, MutexGuard};

#[derive(Debug)]
struct AppState {
    running: bool,
    suspended: bool,
}
static APP_STATE: Mutex<AppState> = Mutex::new(AppState {
    running: true,
    suspended: false,
});

fn access_app_state() -> MutexGuard<'static, AppState> {
    APP_STATE.lock().unwrap()
}

fn confirm_app_state_valid() {
    let app_state = access_app_state();
    if app_state.running == app_state.suspended {
        panic!("Fatal! App was both running and suspended at same time...");
    }
    println!("App state is correct: {:?}", app_state);
}

fn change_app_state() {
    let mut app_state = access_app_state();
    app_state.running = !app_state.running;
    app_state.suspended = !app_state.suspended;
}
