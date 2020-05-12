use super::ViewPage;
use seed::prelude::*;
use seed::*;

pub fn view<Ms>() -> ViewPage<Ms> {
    ViewPage::new("Page Not Found", view_content())
}

fn view_content<Ms>() -> Node<Ms> {
    main![id!("content"), class!["container"], attrs! {At::TabIndex => -1}, h1!["Not Found"],]
}
