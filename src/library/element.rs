use super::{js_packet::JSPacket, tag::Tag};

pub trait Element {
    fn get_html(&self) -> String;
    
    fn style(&mut self, style: &str) -> &dyn Element;

    fn onclick(&mut self, js_event: JSPacket) -> &dyn Element;

    fn add_tag(&mut self, tag: Tag) -> &dyn Element;
}
