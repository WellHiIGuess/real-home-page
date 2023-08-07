use super::{element::Element, js_packet::JSPacket, tag::Tag};

pub struct Header {
    pub h_type: String,
    pub text: String,
    pub tags: Vec<Tag>
}

impl Header {
    #[allow(unused)]
    pub fn new(h_type: &str, text: &str) -> Header {
        Header {
            h_type: h_type.to_string(),
            text: text.to_string(),
            tags: vec![],
        }
    }
}

impl Element for Header {
    fn get_html(&self) -> String {
        let mut tag_impl = String::new();

        for i in &self.tags {
            tag_impl += format!(" {}=\"{}\" ", i.name, i.content).as_str();
        }

        "<h".to_owned() + self.h_type.as_str() + tag_impl.as_str() + ">" + self.text.as_str() + "</h" + self.h_type.as_str() + ">"
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
