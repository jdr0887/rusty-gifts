use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use seed::prelude::*;
use seed::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    problems: Vec<(String, String)>,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    password: String,
    confirm_password: String,
    base_url: Url,
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
    Model { session, ..Model::default() }
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
    PasswordChanged(String),
    RegisterSubmitted,
    RegisterCancelled,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::EmailChanged(email) => model.email = email,
        // Msg::FirstNameChanged(first_name) => model.first_name = Some(first_name),
        // Msg::LastNameChanged(last_name) => model.last_name = Some(last_name),
        Msg::PasswordChanged(password) => model.password = password,
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
            class!["col-md-6", "offset-md-3" "mt-5"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Gift App Profile",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["First Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.email, },
                                input_ev(Ev::Input, Msg::EmailChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Last Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.email, },
                                input_ev(Ev::Input, Msg::EmailChanged)
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
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.email, },
                                input_ev(Ev::Input, Msg::EmailChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Password"],
                            input![
                                attrs! { At::Type => "password", At::Class => "form-control", At::Value => model.password, },
                                input_ev(Ev::Input, Msg::PasswordChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Confirm Password"],
                            input![
                                attrs! { At::Type => "password", At::Class => "form-control", At::Value => model.password, },
                                input_ev(Ev::Input, Msg::PasswordChanged)
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
