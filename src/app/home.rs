use crate::{library::{page::{page, Page}, css_packet::CSSPacket}, elements::hello::Hello};

pub fn home() -> Page {
    page(vec![
        &Hello {},
    ]).add_style_sheet(CSSPacket::new("style.css"))
}
