use std::env;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod extractor;
mod view;

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_port", "12500");
    env::set_var("ROCKET_address", "0.0.0.0");

    let rocket = rocket::build();

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = view::api();
    let (extractor_path, extractor_routes) = view::ex_api();
    rocket
        .mount(assets_path, asset_routes)
        .mount(body_path, body_routes)
        .mount(extractor_path, extractor_routes)
}
