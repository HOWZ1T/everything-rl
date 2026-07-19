pub mod rgl;

use glfw;
use glfw::{WindowEvent};

fn render() {

}

fn event_handler(window: &mut glfw::Window, event: WindowEvent) {

}

fn main() {
    let res = rgl::App::new(800, 600, "Everything RL", render, event_handler);
    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }
    let app = res.ok().unwrap();
    app.run().expect("TODO: panic message");
}
