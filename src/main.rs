extern crate pencil;
extern crate motivations;
extern crate pick_one;

use std::env;
use std::str::FromStr;

use pencil::Pencil;

mod routes;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(7878)
}

fn main() {
    let mut app = Pencil::new("/web/motivation");
    app.get("/", "motivation", routes::motivation);
    
    let host = "0.0.0.0";
    let port = get_server_port();
    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}", address);
    app.run(&address[..]);
}
