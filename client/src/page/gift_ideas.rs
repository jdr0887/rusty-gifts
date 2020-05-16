use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;
use seed::*;
use std::collections;

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    problems: collections::HashMap<String, String>,
    gift_ideas: Vec<shared::GiftIdeaResponseBody>,
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

pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    let user = LocalStorage::get(crate::STORAGE_KEY).ok();
    let session = Session::new(user.clone());
    if session.viewer().is_none() {
        route::go_to(route::Route::Login, orders);
    }

    let find_all_gifts_request = Request::new("/v1/gifts/find_all").method(Method::Get);
    orders.perform_cmd(async { Msg::GiftIdeasLoaded(async { find_all_gifts_request.fetch().await?.check_status()?.json().await }.await) });

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
pub enum Msg {
    ReserveGiftIdea(i32),
    UnreserveGiftIdea(i32),
    GiftIdeaReserved(fetch::Result<shared::GiftIdeaResponseBody>),
    GiftIdeaUnreserved(fetch::Result<shared::GiftIdeaResponseBody>),
    EditGiftIdea(i32),
    DeleteGiftIdea(i32),
    GiftIdeaDeleted(fetch::Result<bool>),
    GiftIdeasLoaded(fetch::Result<Vec<shared::GiftIdeaResponseBody>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    log!("gift_ideas.update(): model.session.viewer(): ", model.session.viewer());
    match msg {
        Msg::ReserveGiftIdea(gift_id) => {
            log!("ReserveGiftIdea.id: ", gift_id);
            let reserve_gift_request = Request::new(format!("/v1/gifts/reserve/{}/{}", gift_id, model.session.viewer().unwrap().id)).method(Method::Patch);
            orders.perform_cmd(async { Msg::GiftIdeaReserved(async { reserve_gift_request.fetch().await?.check_status()?.json().await }.await) });
            // let mut gift = model.gift_ideas.iter().filter(|e| e.id == gift_id).take(1).nth(0).unwrap();
            // gift.reserved_by_user_id = Some(model.session.viewer().unwrap().id.clone());
            // let find_all_gifts = Request::new("/v1/gifts/find_all").method(Method::Get);
            // orders
            //     .perform_cmd(async { Msg::GiftIdeaReserved(async { reserve_gift_request.fetch().await?.check_status()?.json().await }.await) })
            //     .perform_cmd(async { Msg::GiftIdeasLoaded(async { find_all_gifts.fetch().await?.check_status()?.json().await }.await) });
        }
        Msg::UnreserveGiftIdea(gift_id) => {
            log!("UnreserveGiftIdea.id: ", gift_id);
            let unreserve_gift_request = Request::new(format!("/v1/gifts/unreserve/{}", gift_id)).method(Method::Patch);
            orders.perform_cmd(async { Msg::GiftIdeaUnreserved(async { unreserve_gift_request.fetch().await?.check_status()?.json().await }.await) });
        }
        Msg::EditGiftIdea(gift_id) => {
            log!("EditGiftIdea.id: ", gift_id);
        }
        Msg::DeleteGiftIdea(gift_id) => {
            log!("DeleteGiftIdea.id: ", gift_id);
            let delete_gift_request = Request::new(format!("/v1/gifts/delete/{}", gift_id)).method(Method::Delete);
            orders.perform_cmd(async { Msg::GiftIdeaDeleted(async { delete_gift_request.fetch().await?.check_status()?.json().await }.await) });
            model.gift_ideas.retain(|e| e.id != gift_id);
        }
        Msg::GiftIdeasLoaded(Ok(mut gift_ideas)) => {
            gift_ideas.retain(|e| e.recipient_user_id != model.session.viewer().unwrap().id);
            model.gift_ideas = gift_ideas;
        }
        Msg::GiftIdeasLoaded(Err(fetch_error)) => {
            log!("error loading gift ideas", fetch_error);
            orders.skip();
        }
        Msg::GiftIdeaDeleted(Ok(confirmation)) => {
            log!("GiftIdea was deleted: ", confirmation);
            route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::GiftIdeaDeleted(Err(confirmation)) => {
            log!("GiftIdea was not deleted: ", confirmation);
        }
        Msg::GiftIdeaReserved(Ok(gift_idea)) => {
            let mut gift = model.gift_ideas.iter_mut().filter(|e| e.id == gift_idea.id).take(1).nth(0).unwrap();
            gift.reserved_by_user_id = gift_idea.reserved_by_user_id;
            // route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::GiftIdeaReserved(Err(error)) => log!(error),
        Msg::GiftIdeaUnreserved(Ok(gift_idea)) => {
            route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::GiftIdeaUnreserved(Err(error)) => log!(error),
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    log!("gift_ideas.view(): model.session.viewer(): ", model.session.viewer());
    let user_id = model.session.viewer().unwrap().id;
    ViewPage::new(
        "Gift Ideas",
        div![
            class!["col-md-8", "offset-md-2" "mt-4"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Gift Ideas",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![table![
                        class!["table", "table-striped"],
                        attrs![ At::Width => "100%"],
                        thead![
                            class!["thead-dark"],
                            tr![
                                th![attrs![ At::Scope => "col", At::Width => "20%"], "Title"],
                                th![attrs![ At::Scope => "col",At::Width => "30%"], "Description"],
                                th![attrs![ At::Scope => "col",At::Width => "5%"], "Price"],
                                th![attrs![ At::Scope => "col",At::Width => "25%"], "URL"],
                                th![attrs![ At::Scope => "col",At::Width => "20%"], "Actions"],
                            ],
                        ],
                        tbody![if model.gift_ideas.is_empty() {
                            vec![tr![td![attrs![ At::ColSpan => "5"], "No Data"]]]
                        } else {
                            model.gift_ideas.iter().map(|e| build_row(e.clone(), user_id)).collect()
                        }],
                    ],],
                ],
            ],
        ],
    )
}

pub fn build_row(gift_idea: shared::GiftIdeaResponseBody, user_id: i32) -> seed::virtual_dom::node::Node<Msg> {
    let gift_idea_id = gift_idea.id.clone();
    tr![
        th![attrs![ At::Scope => "row"], gift_idea.title],
        td![gift_idea.description],
        td![gift_idea.price],
        td![gift_idea.url],
        td![
            //   {% if request.user != idea.recipient.user %}
            //     {% if not idea.reserved_by %}
            //       <a href="{% url reserve idea.id %}">reserve</a>
            //     {% else %}
            //       {% if idea.reserved_by == request.user %}
            //         <a href="{% url unreserve idea.id %}">unreserve</a>
            //       {% else %}
            //         <b>reserved</b>
            //       {% endif %}
            //     {% endif %}
            //   {% endif %}<br/>
            match user_id != gift_idea.recipient_user_id {
                true => match gift_idea.reserved_by_user_id {
                    None => a![
                        class!["btn", "btn-primary", "btn-sm", "text-white"],
                        "Reserve",
                        ev(Ev::Click, move |event| {
                            event.prevent_default();
                            Msg::ReserveGiftIdea(gift_idea_id)
                        })
                    ],
                    Some(_) => match gift_idea.reserved_by_user_id {
                        Some(reserved_by_user_id) => match reserved_by_user_id == user_id {
                            true => a![
                                class!["btn", "btn-primary", "btn-sm", "text-white"],
                                "Unreserve",
                                ev(Ev::Click, move |event| {
                                    event.prevent_default();
                                    Msg::UnreserveGiftIdea(gift_idea_id)
                                })
                            ],
                            _ => a![class!["btn", "btn-primary", "btn-sm", "disabled"], "Reserved"],
                        },
                        None => a![class!["btn", "btn-primary", "btn-sm", "disabled"], "Reserved"],
                    },
                },
                _ => empty![],
            },
            match user_id == gift_idea.owner_id {
                true => {
                    vec![
                        button![
                            class!["btn", "btn-primary", "btn-sm", "ml-1"],
                            "Edit",
                            ev(Ev::Submit, move |event| {
                                event.prevent_default();
                                Msg::EditGiftIdea(gift_idea_id)
                            })
                        ],
                        a![
                            class!["btn", "btn-primary", "btn-sm", "text-white", "ml-1"],
                            "Delete",
                            ev(Ev::Click, move |event| {
                                event.prevent_default();
                                Msg::DeleteGiftIdea(gift_idea_id)
                            })
                        ],
                    ]
                }
                _ => {
                    vec![empty![]]
                }
            }
        ],
    ]
}
