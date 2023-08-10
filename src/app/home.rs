use crate::{library::{page::{page, Page}, css_packet::CSSPacket, header::Header, tag::Tag, paragraph::Paragraph, image::Image, element::Element}, elements::top_bar::TopBar};

pub fn home() -> Page {
    let mut hello_header = Header::new("1", "Hello!");
    hello_header.tags = vec![
        Tag::new("class", "navbar-text".to_string()),
        Tag::new("id", "hello-header".to_string()),
    ];

    let mut hello_message = Paragraph::new("I am a random programmer on the internet who needed a website. My name is WellHiIGuess or BENJAMIN! I usually program games and sometimes make horrible looking websites like this.");
    hello_message.tags = vec![
        Tag::new("class", "navbar-text".to_string()),
        Tag::new("id", "hello-message".to_string()),
    ];

    let mut image = Image::new();
    image.tags = vec![
        Tag::new("src", "../pub/thinking.png".to_string()),
        Tag::new("id", "thinking-image".to_string()),
        Tag::new("class", "fade-in".to_string()),
    ];

    page(vec![
        &TopBar { },
        &hello_header,
        &hello_message,
        &image,
    ]).add_style_sheet(CSSPacket::new("style.css"))
}
