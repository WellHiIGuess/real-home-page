use super::{element::Element, js_packet::JSPacket, tag::Tag};

pub struct Div<'a> {
    pub elements: Vec<Box<&'a dyn Element>>,
    pub tags: Vec<Tag>,
}

impl Div<'_> {
    #[allow(unused)]
    pub fn new<'a>(elements: Vec<&'a dyn Element>) -> Div<'a> {
        let mut b_elements = vec![];

        for i in elements {
            b_elements.push(Box::new(i));
        }
        
        Div {
            elements: b_elements,
            tags: vec![],
        }
    }
}

impl Element for Div<'_> {
    fn get_html(&self) -> String {
        let mut tag_impl = String::new();

        for i in &self.tags {
            tag_impl += format!(" {}=\"{}\" ", i.name, i.content).as_str();
        }

        "<div".to_owned() + tag_impl.as_str() + ">" + self.elements
            .iter()
            .map(|x| x.get_html())
            .collect::<Vec<_>>()
            .join("").as_str() + "</div>"
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
