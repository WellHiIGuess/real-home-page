use super::{element::Element, js_packet::JSPacket};

pub struct Paragraph {
    pub text: String,
    js_event: Option<JSPacket>,
    pub style: Option<String>,
}

impl Paragraph {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            js_event: None,
            style: None,
        }
    }
}

impl Element for Paragraph {
    fn get_html(&self) -> String {
        let mut style_imp = String::new();

        if self.style != None {
            style_imp = " style=\"".to_owned() + self.style.as_ref().unwrap().as_str() + "\" ";
        }

        "<p".to_owned() + &style_imp + ">" + self.text.as_str() + "</p>"
    }

    fn style(&mut self, style: &str) -> &dyn Element {
        self.style = Some(style.to_string());
        self
    }

    fn onclick(&mut self, js_event: JSPacket) -> &dyn Element {
        self.js_event = Some(js_event);
        self
    }
}
