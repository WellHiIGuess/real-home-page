use super::{element::Element, js_packet::JSPacket};

pub struct Button {
    pub text: String,
    pub js_event: Option<JSPacket>,
    pub style: Option<String>,
}

impl Button {
    #[allow(unused)]
    pub fn new(text: &str, js_event: Option<JSPacket>) -> Button {
        Button {
            text: text.to_string(),
            js_event,
            style: None,
        }
    }
}

impl Element for Button {
    fn get_html(&self) -> String {
        let mut style_imp = String::new();
        let mut js_event_imp = &String::new();

        if self.style != None {
            style_imp = " style=\"".to_owned() + self.style.as_ref().unwrap().as_str() + "\" ";
        }

        if self.js_event != None {
            js_event_imp = &self.js_event.as_ref().unwrap().content;
        }

        "<button".to_owned() + &style_imp + " onclick=\"" + js_event_imp + "\">" + self.text.as_str() + "</button" + ">"
    }

    fn style(&mut self, style: &str) -> &dyn Element {
        self.style = Some(style.to_string());
        self
    }
}
