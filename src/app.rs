pub struct App {
    addr: String,
    definitions: String,
    port: u16,
}

impl App {
    pub fn new(addr: String, definitions: String, port: u16) -> Self {
        App{addr, definitions, port}
    }

    pub fn listen(self) {}
}
