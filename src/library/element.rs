use super::{js_packet::JSPacket, tag::Tag};

pub trait Element {
    fn get_html(&self) -> String;
    
    fn add_tag(&mut self, tag: Tag) -> &dyn Element;

    fn add_action_tag(&mut self, name: &str, js_event: JSPacket) -> &dyn Element;
}
