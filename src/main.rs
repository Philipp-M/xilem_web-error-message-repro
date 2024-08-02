use xilem_web::{document_body, elements::svg, interfaces::*, svg::peniko::Color, App};

pub fn main() {
    App::new(document_body(), (), |_| {
        svg::line(()).fill(Color::LIGHT_GRAY)
    })
    .run();
}
