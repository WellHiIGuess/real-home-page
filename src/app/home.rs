use crate::{library::{page::{page, Page}, button::Button, js_packet::JSPacket}, elements::hello::Hello};

pub fn home() -> Page {
    page(vec![
        &Hello {},
        &Button::new("Click me", Some(JSPacket::new("test.js"))),
    ])
}
