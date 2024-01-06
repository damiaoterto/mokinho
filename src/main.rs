mod app;

use clap::Parser;
use app::App;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    addr: String,

    #[arg(short, long, default_value_t = 9090)]
    port: u16,

    #[arg(short, long, default_value_t = String::from("definitions.json"))]
    definitions: String,
}

fn main() {
    let args = Args::parse();
    let app = App::new(args.addr, args.definitions, args.port);
    
    app.listen();
}
