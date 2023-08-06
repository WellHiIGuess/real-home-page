use super::js_packet::JSPacket;

pub trait Element {
    fn get_html(&self) -> String;
    
    fn style(&mut self, style: &str) -> &dyn Element;

    fn onclick(&mut self, js_event: JSPacket) -> &dyn Element;
}
