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

    fn add_tag(&mut self, tag: Tag) -> &dyn Element {
        self.tags.push(tag);
        self
    }

    fn add_action_tag(&mut self, name: &str, js_event: JSPacket) -> &dyn Element {
        self.tags.push(Tag::new(name, ("console.log('getting stuff');async function get() {const response = await fetch('../$get_js/".to_owned() + &js_event.path + "').then(response => response.text()).then(data=>eval(data));}get();").to_string()));
        self
    }
}
