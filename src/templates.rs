extern crate pencil;
extern crate motivations;
extern crate pick_one;

use std::collections::BTreeMap;

use motivations::MOTIVATIONS;

use pencil::{Request, PencilResult};

pub fn motivation(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    let motivation = pick_one::pick_one_str(&MOTIVATIONS).to_string();
    context.insert("motivation".to_string(), motivation);
    return request.app.render_template("motivation.html", &context);
}
