use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;
use seed::*;
use std::collections;

#[derive(Default, Clone, Debug)]
pub struct Form {
    title: String,
    description: String,
    price: String,
    url: String,
    owner_id: i32,
    recipient_user_id: i32,
}

impl From<Form> for shared::GiftIdeaRequestBody {
    fn from(form: Form) -> shared::GiftIdeaRequestBody {
        shared::GiftIdeaRequestBody {
            title: form.title,
            description: Some(form.description),
            price: Some(form.price),
            url: Some(form.url),
            owner_id: form.owner_id,
            recipient_user_id: form.recipient_user_id,
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    recipients: Vec<shared::MinimalUserInfo>,
    problems: collections::HashMap<String, String>,
    form: Form,
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

    let request = Request::new("/v1/users/find_all").method(Method::Get);
    orders.perform_cmd(async { Msg::RecipientNamesLoaded(async { request.fetch().await?.check_status()?.json().await }.await) });

    let mut model = Model::default();
    model.form.owner_id = session.viewer().unwrap().id.clone();
    Model { session, ..model }
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
    TitleChanged(String),
    DescriptionChanged(String),
    PriceChanged(String),
    URLChanged(String),
    RecipientChanged(String),
    AddGiftIdeaSubmitted,
    AddGiftIdeaFetched(fetch::Result<shared::GiftIdeaResponseBody>),
    AddGiftIdeaCancelled,
    RecipientNamesLoaded(fetch::Result<Vec<shared::MinimalUserInfo>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::TitleChanged(title) => model.form.title = title,
        Msg::DescriptionChanged(description) => model.form.description = description,
        Msg::PriceChanged(price) => model.form.price = price,
        Msg::URLChanged(url) => model.form.url = url,
        Msg::RecipientChanged(recipient_user_id) => {
            model.form.recipient_user_id = recipient_user_id.parse::<i32>().unwrap();
        }
        Msg::AddGiftIdeaCancelled => route::go_to(route::Route::GiftIdeas, orders),
        Msg::AddGiftIdeaSubmitted => {
            if !model.form.title.is_empty() && model.form.recipient_user_id != 0 {
                let request = Request::new("/v1/gifts/add")
                    .method(Method::Post)
                    .json::<shared::GiftIdeaRequestBody>(&model.form.clone().into());
                orders.perform_cmd(async { Msg::AddGiftIdeaFetched(async { request?.fetch().await?.check_status()?.json().await }.await) });
            }
        }
        Msg::AddGiftIdeaFetched(Ok(gift)) => {
            route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::AddGiftIdeaFetched(Err(error)) => log!(error),
        Msg::RecipientNamesLoaded(Ok(recipients)) => {
            model.recipients = recipients;
        }
        Msg::RecipientNamesLoaded(Err(error)) => log!(error),
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(
        "Add Gift Idea",
        div![
            class!["col-md-4", "offset-md-4" "mt-4"],
            div![
                attrs![At::Class => "card"],
                h4![attrs! { At::Class => "card-header" }, "Add Gift Idea",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![
                        div![
                            attrs! { At::Class => "form-group" },
                            label![class!["control-label"], "Recipient"],
                            select![
                                class!["form-control"],
                                option![attrs![ At::Value => "0" ], "Select One"],
                                model
                                    .recipients
                                    .iter()
                                    .map(|e| option![
                                        attrs! [ At::Value => e.id.to_string() ],
                                        format!("{} {}", e.first_name.as_ref().unwrap_or(&"".to_string()), e.last_name.as_ref().unwrap_or(&"".to_string()))
                                    ])
                                    .collect::<Vec<seed::virtual_dom::node::Node<Msg>>>(),
                                input_ev(Ev::Input, Msg::RecipientChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Title"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.form.title, },
                                input_ev(Ev::Input, Msg::TitleChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label![class!["control-label"], "Description"],
                            textarea![
                                class!["form-control"],
                                attrs! { At::Type => "textarea", At::Value => model.form.description, },
                                input_ev(Ev::Input, Msg::DescriptionChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label![class!["control-label"], "Price"],
                            input![
                                class!["form-control"],
                                attrs! { At::Type => "text", At::Value => model.form.price, },
                                input_ev(Ev::Input, Msg::PriceChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label![class!["control-label"], "URL"],
                            input![
                                class!["form-control"],
                                attrs! { At::Type => "text", At::Value => model.form.url },
                                input_ev(Ev::Input, Msg::URLChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            button![
                                class!["btn", "btn-primary"],
                                "Submit",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::AddGiftIdeaSubmitted
                                })
                            ],
                            a![
                                class!["btn", "btn-primary", "text-white", "ml-1"],
                                "Cancel",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::AddGiftIdeaCancelled
                                })
                            ],
                        ],
                    ],
                ],
            ],
        ],
    )
}
