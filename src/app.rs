use std::net::Ipv4Addr;

pub struct App {
    addr: String,
    definitions: String,
    port: u16,
}

impl App {
    pub fn new(addr: String, definitions: String, port: u16) -> Self {
        App {
            addr,
            definitions,
            port,
        }
    }

    fn addr_str_parser(&self, addr: String) -> Ipv4Addr {
        match addr.parse::<Ipv4Addr>() {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid address: {}", e),
        }
    }

    pub fn listen(&self) {
        let _ = self.addr_str_parser(self.addr.clone());
    }
}
