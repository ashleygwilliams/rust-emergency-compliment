use handlebars::Handlebars;
use motivations::MOTIVATIONS;
use pick_one;
use std::collections::BTreeMap;

pub fn motivation() -> Vec<u8> {
    let mut context = BTreeMap::new();
    let motivation = pick_one::pick_one_str(&MOTIVATIONS).to_string();
    let crab = pick_one::pick_one_str(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"]).to_string();
    context.insert("motivation".to_string(), motivation);
    context.insert("image".to_string(), crab);

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("motivation", "src/templates/motivation.html").unwrap();

    handlebars.render("motivation", &context).unwrap().into_bytes()
}
