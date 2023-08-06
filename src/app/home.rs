use crate::{library::{page::{page, Page}, button::Button, js_packet::JSPacket, element::Element}, elements::hello::Hello};

pub fn home() -> Page {
    page(vec![
        &Hello {},
    ])
}
