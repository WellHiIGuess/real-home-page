use crate::library::{element::Element, paragraph::Paragraph};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Paragraph::new("Hello").style("color: red;").get_html()
    }

    fn style(&mut self, _: &str) -> &dyn Element {
        self
    }
}
