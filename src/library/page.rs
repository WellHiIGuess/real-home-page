use super::{element::Element, css_packet::CSSPacket};

#[derive(Clone)]
pub struct Page {
    html: String,
    style_sheet: Option<CSSPacket>,
}

impl Page {
    pub fn new(html: String) -> Self {
        Self {
            html,
            style_sheet: None,
        }
    }

    pub fn serve(self) -> String {
        self.html + format!("<style>{}</style>",self.style_sheet.unwrap_or(CSSPacket::default()).content).as_str()
    }

    pub fn add_style_sheet(&mut self, style_sheet: CSSPacket) -> Page {
        self.style_sheet = Some(style_sheet);
        self.clone()
    }
}

pub fn page(elements: Vec<&dyn Element>) -> Page {
    let output: String = elements
        .iter()
        .map(|&m| m.get_html())
        .collect();

    Page::new(output)
}
