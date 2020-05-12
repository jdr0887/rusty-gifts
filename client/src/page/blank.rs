use super::ViewPage;
use seed::*;

pub fn view<Ms>() -> ViewPage<Ms> {
    ViewPage::new("Blank", empty!())
}
