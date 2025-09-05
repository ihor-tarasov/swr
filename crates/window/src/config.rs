#[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            title: "SWR Window",
            width: 640,
            height: 480,
        }
    }
}
