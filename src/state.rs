use std::sync::Mutex;

#[derive(Debug)]
pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}
