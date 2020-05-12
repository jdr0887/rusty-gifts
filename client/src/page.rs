use crate::route::Route;
use crate::LoggedUser;
use seed::prelude::*;
use seed::*;

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
}

#[allow(clippy::unused_self)]
impl Page {
    fn is_active(&self, route: &Route) -> bool {
        match (self, route) {
            (Page::Home, Route::Home) | (Page::Login, Route::Login) | (Page::Register, Route::Register) | (Page::Profile, Route::Profile) | (Page::GiftIdeas, Route::GiftIdeas) => {
                true
            }
            _ => false,
        }
    }

    // ------ view methods ------
    pub fn view<Ms>(&self, view_page: ViewPage<Ms>, viewer: Option<&LoggedUser>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());
        // vec![self.view_header(viewer), view_page.into_content()]
        vec![view_page.into_content()]
    }

    // ====== PRIVATE ======

    fn view_header<Ms>(&self, viewer: Option<&LoggedUser>) -> Node<Ms> {
        nav![
            class!["navbar", "navbar-light"],
            div![
                class!["container"],
                a![class!["navbar-brand"], attrs! {At::Href => Route::Home.to_string()}, "Home"],
                ul![
                    class!["nav navbar-nav pull-xs-right"],
                    match viewer {
                        //None => vec![self.view_navbar_link(&Route::Login, "Login"), self.view_navbar_link(&Route::Register, "Register")],
                        None => vec![self.view_navbar_link(&Route::Login, "Login"), self.view_navbar_link(&Route::Register, "Register")],
                        Some(viewer) => vec![
                            self.view_navbar_link(&Route::Home, "Home"),
                            self.view_navbar_link(&Route::Profile, "Profile"),
                            self.view_navbar_link(&Route::Logout, "Logout"),
                        ],
                    },
                ],
            ]
        ]
    }

    fn view_navbar_link<Ms>(&self, route: &Route, link_content: &str) -> Node<Ms> {
        li![
            class!["nav-item"],
            a![
                class![
                    "nav-link",
                    "active" => self.is_active(route),
                ],
                attrs! {At::Href => route.to_string()},
                link_content
            ]
        ]
    }
}
