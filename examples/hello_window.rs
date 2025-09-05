use swr::prelude::*;

struct HelloWindow;

impl State for HelloWindow {
    fn render(&mut self, mut buffer: ColorBuffer) {
        buffer.clear(vec4(0.2, 0.3, 0.3, 1.0));
    }
}

fn main() {
    run(
        Config {
            title: "Hello SWR Window",
            width: 800,
            height: 600,
        },
        HelloWindow,
    )
    .unwrap();
}
