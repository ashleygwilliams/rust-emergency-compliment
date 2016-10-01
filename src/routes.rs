extern crate pencil;
extern crate motivations;
extern crate pick_one;

use motivations::MOTIVATIONS;

use pencil::{Request, Response, PencilResult};

pub fn motivation(_: &mut Request) -> PencilResult {
    Ok(Response::from(pick_one::pick_one_str(&MOTIVATIONS)))
}
