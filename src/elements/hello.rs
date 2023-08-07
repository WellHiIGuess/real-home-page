use crate::library::{element::Element, paragraph::Paragraph, div::Div, button::Button, js_packet::JSPacket};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Div::new(vec![
            &Paragraph::new("Hello"),
            Button::new("Click me")
                .add_action_tag("onclikc", JSPacket::new("test.js"))
        ]).get_html()
    }

    fn add_tag(&mut self, _: crate::library::tag::Tag) -> &dyn Element {
        self
    }

    fn add_action_tag(&mut self, _: &str, _: JSPacket) -> &dyn Element {
        self
    }
}
