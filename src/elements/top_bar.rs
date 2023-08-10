use crate::library::{element::Element, div::Div, tag::Tag, paragraph::Paragraph, js_packet::JSPacket};

pub struct TopBar { }

impl Element for TopBar {
    fn get_html(&self) -> String {
        let mut home_text = Paragraph::new("WellHiIGuess");
        home_text.tags = vec![
            Tag::new("class", "navbar-text".to_string()),
            Tag::new("id", "home-text".to_string()),
        ];

        Div::new(vec![
            home_text.add_action_tag("onclick", JSPacket::new("home.js")),
        ])
        .add_tag(Tag::new("class", "top-bar".to_string()))
        .get_html()
    }

    fn add_tag(&mut self, _tag: crate::library::tag::Tag) -> &dyn Element {
        self
    }

    fn add_action_tag(&mut self, _name: &str, _js_event: crate::library::js_packet::JSPacket) -> &dyn Element {
        self
    }
}
