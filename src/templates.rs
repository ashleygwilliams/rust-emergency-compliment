extern crate pencil;
extern crate motivations;
extern crate pick_one;

use std::collections::BTreeMap;

use motivations::MOTIVATIONS;

use pencil::{Request, PencilResult};

pub fn motivation(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    let motivation = pick_one::pick_one_str(&MOTIVATIONS).to_string();
    let crab = pick_one::pick_one_str(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"]).to_string();
    context.insert("motivation".to_string(), motivation);
    context.insert("image".to_string(), crab);
    return request.app.render_template("motivation.html", &context);
}
