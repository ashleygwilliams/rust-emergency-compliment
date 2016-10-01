extern crate pencil;
extern crate motivations;
extern crate pick_one;

use std::env;

use pencil::Pencil;

mod templates;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(7878)
}

fn main() {
    let mut app = Pencil::new("./src");
 
    app.enable_static_file_handling();   
    app.register_template("motivation.html");

    app.get("/", "motivation", templates::motivation);

    let host = "0.0.0.0";
    let port = get_server_port();
    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}", address);
    app.run(&address[..]);
}
