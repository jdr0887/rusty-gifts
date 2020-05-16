use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;
use seed::*;

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
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

    let user = user.unwrap();
    let mut model = Model::default();
    model.session = session;
    model.first_name = user.first_name;
    model.last_name = user.last_name;
    model.email = user.email;
    // model.phone = user.phone.unwrap_or("");

    model
}

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(route::Route::Home, orders);
        }
        _ => (),
    }
}

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Msg {
    EmailChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    PhoneChanged(String),
    RegisterSubmitted,
    RegisterCancelled,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::EmailChanged(email) => model.email = email,
        Msg::FirstNameChanged(first_name) => model.first_name = first_name,
        Msg::LastNameChanged(last_name) => model.last_name = last_name,
        Msg::PhoneChanged(phone) => model.phone = phone,
        Msg::RegisterSubmitted => {
            // orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).register()));
        }
        Msg::RegisterCancelled => {
            // orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).login()));
        } // Msg::FormSubmitted => match model.form.trim_fields().validate() {
          //     Ok(valid_form) => {
          //         model.problems.clear();
          //         orders.perform_cmd(request::register::register(
          //             &valid_form,
          //             Msg::RegisterCompleted,
          //         ));
          //     }
          //     Err(problems) => {
          //         model.problems = problems;
          //     }
          // },
          // Msg::FieldChanged(field) => {
          //     model.form.upsert_field(field);
          // }
          // Msg::RegisterCompleted(Ok(viewer)) => {
          //     viewer.store();
          //     orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(viewer)));
          // }
          // Msg::RegisterCompleted(Err(problems)) => {
          //     model.problems = problems;
          // }
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(
        "Profile",
        div![
            class!["col-md-6", "offset-md-3" "mt-4"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Profile",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["First Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.first_name, },
                                input_ev(Ev::Input, Msg::FirstNameChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Last Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.last_name, },
                                input_ev(Ev::Input, Msg::LastNameChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Email"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.email, },
                                input_ev(Ev::Input, Msg::EmailChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Phone"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.phone, },
                                input_ev(Ev::Input, Msg::PhoneChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            button![
                                class!["btn", "btn-primary"],
                                "Submit",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::RegisterSubmitted
                                })
                            ],
                            button![
                                class!["btn", "btn-primary", "ml-2"],
                                "Cancel",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::RegisterCancelled
                                })
                            ],
                        ],
                    ],
                ],
            ],
        ],
    )
}
