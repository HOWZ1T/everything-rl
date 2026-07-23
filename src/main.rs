pub mod rgl;

use glfw;
use glfw::{WindowEvent};
use rgl::AppCallbacks;

struct AppState {
    counter: i32,
    t0: f64,
    t1: f64
}

struct MyApp;

impl AppCallbacks for MyApp {
    type State = AppState;

    fn render(&mut self, state: &mut AppState) {

    }

    fn event(&mut self, window: &mut glfw::Window, event: WindowEvent, state: &mut AppState) {

    }

    fn update(&mut self, state: &mut AppState, delta_ms: f64) {
        state.t1 += delta_ms;
        if state.t1 - state.t0 >= 1000.0 {
            state.t0 = 0.0;
            state.t1 = 0.0;
            state.counter += 1;
            println!("counter: {}", state.counter);
        }
    }
}

fn main() {
    let res = rgl::App::new(
        800, 600, "Everything RL",
        AppState { counter: 0, t0: 0.0, t1: 0.0 },
        MyApp,
    );
    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }
    let mut app = res.ok().unwrap();
    app.set_clear_color([1.0, 0.0, 1.0, 1.0]);
    app.run().expect("TODO: panic message");
}
