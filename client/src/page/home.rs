use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use crate::Route;
use seed::app::Orders;
use seed::prelude::*;
use seed::*;

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
}

impl Model {
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        model.session
    }
}

pub fn init(session: Session) -> Model {
    Model { session, ..Model::default() }
}

pub fn sink(g_msg: GMsg, model: &mut Model) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
        }
        _ => (),
    }
}

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Msg {
    LoginClicked,
    RegisterClicked,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::LoginClicked => route::go_to(Route::Login, orders),
        Msg::RegisterClicked => route::go_to(Route::Register, orders),
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(
        "Home",
        div![
            class!["col-md-6", "offset-md-3" "mt-5"],
            div![
                class!["jumbotron"],
                h2!["Welcome to the Gift App"],
                p!["This application intends to organize/manage gifts across users for events like birthdays or christmas."],
                button![
                    class!["btn", "btn-primary"],
                    "Login",
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::LoginClicked
                    })
                ],
                button![
                    class!["btn", "btn-primary", "ml-1"],
                    "Register",
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::RegisterClicked
                    })
                ],
            ]
        ],
    )
}
