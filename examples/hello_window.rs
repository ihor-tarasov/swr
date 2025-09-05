use swr::prelude::*;

struct HelloWindow;

impl State for HelloWindow {
    fn render(&mut self, mut frame: Frame) {
        frame.clear(Some(vec4(0.2, 0.3, 0.3, 1.0)), None);
    }
}

fn main() {
    run(
        Config {
            title: "Hello SWR Window",
            width: 800,
            height: 600,
            ..Default::default()
        },
        HelloWindow,
    )
    .unwrap();
}
