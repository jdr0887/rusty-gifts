#![allow(clippy::must_use_candidate)]
use crate::route::Route;
use page::Page;
use seed::prelude::*;
use seed::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::mem::take;

pub use session::Session;

mod page;
mod route;
mod session;

const STORAGE_KEY: &str = "gift_app";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
enum Model {
    Redirect(Session),
    NotFound(Session),
    Home(page::home::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Profile(page::profile::Model),
    GiftIdeas(page::gift_ideas::Model),
    AddGiftIdea(page::add_gift_idea::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Redirect(Session::default())
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        match model {
            Model::Redirect(session) | Model::NotFound(session) => session,
            Model::Home(model) => model.into(),
            Model::Login(model) => model.into(),
            Model::Register(model) => model.into(),
            Model::Profile(model) => model.into(),
            Model::GiftIdeas(model) => model.into(),
            Model::AddGiftIdea(model) => model.into(),
        }
    }
}

pub enum GMsg {
    RoutePushed(Route),
    SessionChanged(Session),
}

fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    if let GMsg::RoutePushed(ref route) = g_msg {
        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
    }
    match model {
        Model::NotFound(_) | Model::Redirect(_) => {
            if let GMsg::SessionChanged(session) = g_msg {
                *model = Model::Redirect(session);
                route::go_to(Route::Home, orders);
            }
        }
        Model::Home(model) => page::home::sink(g_msg, model),
        Model::Login(model) => page::login::sink(g_msg, model, &mut orders.proxy(Msg::LoginMsg)),
        Model::Register(model) => page::register::sink(g_msg, model, &mut orders.proxy(Msg::RegisterMsg)),
        Model::Profile(model) => page::profile::sink(g_msg, model, &mut orders.proxy(Msg::ProfileMsg)),
        Model::GiftIdeas(model) => page::gift_ideas::sink(g_msg, model, &mut orders.proxy(Msg::GiftIdeasMsg)),
        Model::AddGiftIdea(model) => page::add_gift_idea::sink(g_msg, model, &mut orders.proxy(Msg::AddGiftIdeaMsg)),
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum Msg {
    RouteChanged(Option<Route>),
    HomeMsg(page::home::Msg),
    LoginMsg(page::login::Msg),
    RegisterMsg(page::register::Msg),
    ProfileMsg(page::profile::Msg),
    GiftIdeasMsg(page::gift_ideas::Msg),
    AddGiftIdeaMsg(page::add_gift_idea::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::HomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::HomeMsg));
            }
        }
        Msg::LoginMsg(module_msg) => {
            if let Model::Login(module_model) = model {
                page::login::update(module_msg, module_model, &mut orders.proxy(Msg::LoginMsg));
            }
        }
        Msg::RegisterMsg(module_msg) => {
            if let Model::Register(module_model) = model {
                page::register::update(module_msg, module_model, &mut orders.proxy(Msg::RegisterMsg));
            }
        }
        Msg::ProfileMsg(module_msg) => {
            if let Model::Profile(module_model) = model {
                page::profile::update(module_msg, module_model, &mut orders.proxy(Msg::ProfileMsg));
            }
        }
        Msg::GiftIdeasMsg(module_msg) => {
            if let Model::GiftIdeas(module_model) = model {
                page::gift_ideas::update(module_msg, module_model, &mut orders.proxy(Msg::GiftIdeasMsg));
            }
        }
        Msg::AddGiftIdeaMsg(module_msg) => {
            if let Model::AddGiftIdea(module_model) = model {
                page::add_gift_idea::update(module_msg, module_model, &mut orders.proxy(Msg::AddGiftIdeaMsg));
            }
        }
    }
}

fn change_model_by_route(route: Option<Route>, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    let mut session = || Session::from(take(model));
    match route {
        None => *model = Model::NotFound(session()),
        Some(route) => match route {
            Route::Logout => {
                LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
                orders.send_g_msg(GMsg::SessionChanged(Session::Guest));
                route::go_to(Route::Home, orders)
            }
            Route::Home => {
                *model = Model::Home(page::home::init(session()));
            }
            Route::Login => {
                *model = Model::Login(page::login::init(session()));
            }
            Route::Register => {
                *model = Model::Register(page::register::init(session()));
            }
            Route::Profile => {
                *model = Model::Profile(page::profile::init(session(), &mut orders.proxy(Msg::ProfileMsg)));
            }
            Route::GiftIdeas => {
                *model = Model::GiftIdeas(page::gift_ideas::init(session(), &mut orders.proxy(Msg::GiftIdeasMsg)));
            }
            Route::AddGiftIdea => {
                *model = Model::AddGiftIdea(page::add_gift_idea::init(session(), &mut orders.proxy(Msg::AddGiftIdeaMsg)));
            }
        },
    };
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match model {
        Model::Redirect(session) => Page::Other.view(page::blank::view(), session.viewer()),
        Model::NotFound(session) => Page::Other.view(page::not_found::view(), session.viewer()),
        Model::Home(model) => Page::Home.view(page::home::view(model), model.session().viewer()).map_msg(Msg::HomeMsg),
        Model::Login(model) => Page::Login.view(page::login::view(model), model.session().viewer()).map_msg(Msg::LoginMsg),
        Model::Register(model) => Page::Register.view(page::register::view(model), model.session().viewer()).map_msg(Msg::RegisterMsg),
        Model::Profile(model) => Page::Profile.view(page::profile::view(model), model.session().viewer()).map_msg(Msg::ProfileMsg),
        Model::GiftIdeas(model) => Page::GiftIdeas.view(page::gift_ideas::view(model), model.session().viewer()).map_msg(Msg::GiftIdeasMsg),
        Model::AddGiftIdea(model) => Page::AddGiftIdea.view(page::add_gift_idea::view(model), model.session().viewer()).map_msg(Msg::AddGiftIdeaMsg),
    }
}

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

fn after_mount(url: Url, orders: &mut impl Orders<Msg, GMsg>) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));
    let user = LocalStorage::get(STORAGE_KEY).ok();
    let model = Model::Redirect(Session::new(user));
    AfterMount::new(model).url_handling(UrlHandling::None)
}

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
}
