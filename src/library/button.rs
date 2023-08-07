use super::{element::Element, js_packet::JSPacket, tag::Tag};

pub struct Button {
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Button {
    #[allow(unused)]
    pub fn new(text: &str) -> Button {
        Button {
            text: text.to_string(),
            tags: vec![],
        }
    }
}

impl Element for Button {
    fn get_html(&self) -> String {
        let mut tag_impl= String::new();

        for i in &self.tags {
            tag_impl += format!(" {}=\"{}\" ", i.name, i.content.replace("\"", "'")).as_str();
        }

        "<button".to_owned() + tag_impl.as_str() + ">" + self.text.as_str() + "</button" + ">"
    }

    // fn onclick(&mut self, js_event: JSPacket) -> &dyn Element {
         // self.tags.push(Tag::new("onclick", js_event.to_string()));
         // This is test code
    //     self.tags.push(Tag::new("onclick", ("console.log('getting stuff');async function get() {const response = await fetch('../$get_js/".to_owned() + &js_event.path + "').then(response => response.text()).then(data=>eval(data));}get();").to_string()));
    //     self
    // }

    fn add_tag(&mut self, tag: Tag) -> &dyn Element {
        self.tags.push(tag);
        self
    }

    fn add_action_tag(&mut self, name: &str, js_event: JSPacket) -> &dyn Element {
        self.tags.push(Tag::new("onclick", ("console.log('getting stuff');async function get() {const response = await fetch('../$get_js/".to_owned() + &js_event.path + "').then(response => response.text()).then(data=>eval(data));}get();").to_string()));
        self
    }
}
