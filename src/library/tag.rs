pub struct Tag {
    pub name: String,
    pub content: String,
}

impl Tag {
    pub fn new(name:&str, content: String) -> Tag {
        Tag {
            name: name.to_string(),
            content,
        }
    }
}
