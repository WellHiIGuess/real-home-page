pub trait Element {
    fn get_html(&self) -> String;
    
    fn style(&mut self, style: &str) -> &dyn Element;
}
