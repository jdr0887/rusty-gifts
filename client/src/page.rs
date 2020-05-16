use crate::route::Route;
use crate::LoggedUser;
use seed::prelude::*;
use seed::*;

pub mod add_gift_idea;
pub mod blank;
pub mod gift_ideas;
pub mod home;
pub mod login;
pub mod not_found;
pub mod profile;
pub mod register;

// pub fn scroll_to_top() {
//     seed::window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.).left(0.).behavior(web_sys::ScrollBehavior::Smooth))
// }

pub fn view_errors<Ms: 'static + Clone>(dismiss_errors: Ms, errors: &[String]) -> Node<Ms> {
    if errors.is_empty() {
        empty![]
    } else {
        div![
            class!["error-messages"],
            style! {
                "position" => "fixed",
                "top" => 0,
                "background" => "rgb(250, 250, 250)",
                "padding" => "20px",
                "border" => "1px solid",
                "z-index" => 9999,
            },
            errors.iter().map(|error| p![error]),
            button![simple_ev(Ev::Click, dismiss_errors), "Ok"]
        ]
    }
}

// ------ ViewPage ------

#[allow(clippy::module_name_repetitions)]
pub struct ViewPage<Ms> {
    title_prefix: String,
    content: Node<Ms>,
}

impl<Ms> ViewPage<Ms> {
    pub fn new(title_prefix: &str, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title_prefix.into(),
            content,
        }
    }
    pub fn title(&self) -> String {
        format!("Gifts - {}", self.title_prefix)
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_content(self) -> Node<Ms> {
        self.content
    }
}

pub enum Page {
    Other,
    Home,
    Login,
    Register,
    Profile,
    GiftIdeas,
    AddGiftIdea,
}

#[allow(clippy::unused_self)]
impl Page {
    pub fn view<Ms>(&self, view_page: ViewPage<Ms>, viewer: Option<&LoggedUser>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());
        vec![self.view_header(viewer), view_page.into_content()]
        // vec![view_page.into_content()]
    }

    fn view_header<Ms>(&self, viewer: Option<&LoggedUser>) -> Node<Ms> {
        match viewer {
            None => empty![],
            Some(viewer) => nav![
                class!["navbar", "navbar-expand-lg", "navbar-dark", "bg-dark"],
                a![class!["navbar-brand"], attrs! {At::Href => Route::GiftIdeas.to_string()}, "Gift Ideas"],
                ul![
                    class!["navbar-nav"],
                    self.view_navbar_link(&Route::AddGiftIdea, "Add Gift Idea"),
                    self.view_navbar_link(&Route::Profile, "Profile"),
                    self.view_navbar_link(&Route::Logout, "Logout"),
                ],
            ],
        }
    }

    fn view_navbar_link<Ms>(&self, route: &Route, link_content: &str) -> Node<Ms> {
        a![
            class![ "nav-item"
                "nav-link",
                "active" => self.is_active(route),
            ],
            attrs! {At::Href => route.to_string()},
            link_content
        ]
    }

    fn is_active(&self, route: &Route) -> bool {
        match (self, route) {
            (Page::Home, Route::Home)
            | (Page::Login, Route::Login)
            | (Page::Register, Route::Register)
            | (Page::Profile, Route::Profile)
            | (Page::GiftIdeas, Route::GiftIdeas)
            | (Page::AddGiftIdea, Route::AddGiftIdea) => true,
            _ => false,
        }
    }
}
