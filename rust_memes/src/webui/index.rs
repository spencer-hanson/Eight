use gotham::state::State;
use std::thread::JoinHandle;

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello, World!")
}

pub fn start_webserver() -> JoinHandle<()> {
    let thread = std::thread::spawn(|| gotham::start("127.0.0.1:8000", || Ok(say_hello)));
    return thread;
}
