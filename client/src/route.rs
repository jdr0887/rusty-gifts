use crate::GMsg;
use std::borrow::Cow;
use std::convert;
use std::fmt;

use seed::prelude::*;

#[derive(Clone, Debug)]
pub enum Route<'a> {
    Home,
    Login,
    Logout,
    Register,
    Profile(Cow<'a, str>),
    GiftIdeas,
}

impl<'a> Route<'a> {
    pub fn path(&self) -> Vec<&str> {
        match self {
            super::Route::Home => vec![],
            super::Route::Login => vec!["login"],
            super::Route::Logout => vec!["logout"],
            super::Route::Register => vec!["register"],
            super::Route::Profile(username) => vec!["profile", username],
            super::Route::GiftIdeas => vec!["gift_ideas"],
        }
    }
}

impl<'a> fmt::Display for Route<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl<'a> From<Route<'a>> for seed::Url {
    fn from(route: Route) -> Self {
        seed::Url::new().set_path(route.path())
        //route.path().into()
    }
}

impl<'a> convert::TryFrom<seed::Url> for Route<'a> {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path().into_iter();

        match path.next().as_ref().map(|e| e.as_str()) {
            None | Some("") => Some(Route::Home),
            Some("login") => Some(Route::Login),
            Some("logout") => Some(Route::Logout),
            Some("register") => Some(Route::Register),
            Some("home") => Some(Route::Home),
            // Some("profile") => Some(Route::Profile),
            Some("profile") => path.next().filter(|username| !username.is_empty()).map(Username::from).map(Cow::Owned).map(Route::Profile),
            Some("gift_ideas") => Some(Route::GiftIdeas),
            _ => None,
        }
        .ok_or(())
    }
}

pub fn go_to<Msg: 'static>(route: Route, orders: &mut impl Orders<Msg, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}
