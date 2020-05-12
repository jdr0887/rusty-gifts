use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use crate::LoggedUser;
use seed::prelude::*;
use seed::*;
use serde::{Deserialize, Serialize};
use shared::GiftIdea;
use std::collections;
use web_sys;

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    problems: collections::HashMap<String, String>,
    gift_ideas: Vec<GiftIdea>,
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

pub fn init<Msg: 'static, GMsg>(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    log!("gift_ideas.init(): ", session.viewer());
    Model { session, ..Model::default() }
}

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            log!("gift_ideas.sink.session.viewer(): ", session.viewer());
            model.session = session;
            route::go_to(route::Route::Login, orders);
        }
        _ => (),
    }
}

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Msg {}

pub fn update<Ms: 'static, GMsg>(msg: Msg, model: &mut Model, orders: &mut impl Orders<Ms, GMsg>) {
    log!("gift_ideas.update(): model.session.viewer(): ", model.session.viewer());
    match msg {}
}

pub fn view<Ms: 'static>(model: &Model) -> ViewPage<Ms> {
    log!("gift_ideas.view(): model.session.viewer(): ", model.session.viewer());
    ViewPage::new(
        "Gift Ideas",
        div![
            class!["col-md-8", "offset-md-2" "mt-5"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Gift App Ideas",],
                div![attrs! { At::Class => "card-body" },],
            ],
        ],
    )
}
