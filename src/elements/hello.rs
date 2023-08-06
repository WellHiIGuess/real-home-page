use crate::library::{element::Element, paragraph::Paragraph, div::Div, button::Button, js_packet::JSPacket};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Div::new(vec![
            Paragraph::new("Hello").style("color: red;"),
            Button::new("Click me")
                .onclick(JSPacket::new("test.js")),
        ]).get_html()
    }

    fn style(&mut self, _: &str) -> &dyn Element {
        self
    }

    fn onclick(&mut self, _: crate::library::js_packet::JSPacket) -> &dyn Element {
        self
    }

    fn add_tag(&mut self, _: crate::library::tag::Tag) -> &dyn Element {
        self
    }
}
