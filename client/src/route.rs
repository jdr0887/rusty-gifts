use crate::GMsg;
use std::convert;
use std::fmt;

use seed::prelude::*;

#[derive(Clone, Debug)]
pub enum Route {
    Home,
    Login,
    Logout,
    Register,
    Profile,
    GiftIdeas,
    AddGiftIdea,
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            super::Route::Home => vec![],
            super::Route::Login => vec!["login"],
            super::Route::Logout => vec!["logout"],
            super::Route::Register => vec!["register"],
            super::Route::Profile => vec!["profile"],
            super::Route::GiftIdeas => vec!["gift_ideas"],
            super::Route::AddGiftIdea => vec!["add_gift_idea"],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl From<Route> for seed::Url {
    fn from(route: Route) -> Self {
        seed::Url::new().set_path(route.path())
        //route.path().into()
    }
}

impl convert::TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path().into_iter();

        match path.next().as_ref().map(|e| e.as_str()) {
            None | Some("") => Some(Route::Home),
            Some("login") => Some(Route::Login),
            Some("logout") => Some(Route::Logout),
            Some("register") => Some(Route::Register),
            Some("home") => Some(Route::Home),
            Some("profile") => Some(Route::Profile),
            Some("gift_ideas") => Some(Route::GiftIdeas),
            Some("add_gift_idea") => Some(Route::AddGiftIdea),
            _ => None,
        }
        .ok_or(())
    }
}

pub fn go_to<Msg: 'static>(route: Route, orders: &mut impl Orders<Msg, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}
