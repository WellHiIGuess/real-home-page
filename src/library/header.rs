use super::element::Element;

pub struct Header {
    pub h_type: String,
    pub text: String,
    pub style: Option<String>,
}

impl Header {
    #[allow(unused)]
    pub fn new(h_type: &str, text: &str) -> Header {
        Header {
            h_type: h_type.to_string(),
            text: text.to_string(),
            style: None,
        }
    }
}

impl Element for Header {
    fn get_html(&self) -> String {
        let mut style_imp = String::new();

        if self.style != None {
            style_imp = " style=\"".to_owned() + self.style.as_ref().unwrap().as_str() + "\" ";
        }

        "<h".to_owned() + self.h_type.as_str() + &style_imp + ">" + self.text.as_str() + "</h" + self.h_type.as_str() + ">"
    }

    fn style(&mut self, style: &str) -> &dyn Element {
        self.style = Some(style.to_string());
        self
    }
}
