extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.get("/", "hello", hello);
    
    let host = "127.0.0.1";
    let port = "7878";

    println!("* Running on http://{}:{}/", host, port);
    app.run("127.0.0.1:7878");
}
