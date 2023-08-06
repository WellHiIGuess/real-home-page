use crate::{library::page::{page, Page}, elements::hello::Hello};

pub fn home() -> Page {
    page(vec![
        &Hello {}
    ])
}
