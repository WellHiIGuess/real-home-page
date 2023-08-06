use super::{element::Element, js_packet::JSPacket, tag::Tag};

pub struct Paragraph {
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Paragraph {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            tags: vec![],
        }
    }
}

impl Element for Paragraph {
    fn get_html(&self) -> String {
        let mut tag_impl = String::new();

        for i in &self.tags {
            tag_impl += format!(" {}=\"{}\" ", i.name, i.content).as_str();
        }

        "<p".to_owned() + tag_impl.as_str() + ">" + self.text.as_str() + "</p>"
    }

    fn style(&mut self, style: &str) -> &dyn Element {
        self.tags.push(Tag::new("style", style.to_string()));
        self
    }

    fn onclick(&mut self, js_event: JSPacket) -> &dyn Element {
        self.tags.push(Tag::new("onclick", js_event.to_string()));
        self
    }

    fn add_tag(&mut self, tag: Tag) -> &dyn Element {
        self.tags.push(tag);
        self
    }
}
