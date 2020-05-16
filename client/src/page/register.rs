use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use crate::LoggedUser;
use seed::prelude::*;
use seed::*;
use std::collections;

#[derive(Default, Clone, Debug)]
pub struct Form {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    password: String,
    confirm_password: String,
}

impl From<Form> for shared::RegisterRequestBody {
    fn from(form: Form) -> shared::RegisterRequestBody {
        shared::RegisterRequestBody {
            email: form.email,
            password: form.password,
            first_name: Some(form.first_name),
            last_name: Some(form.last_name),
            phone: Some(form.phone),
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
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

pub fn init(session: Session) -> Model {
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
    FirstNameChanged(String),
    LastNameChanged(String),
    PhoneChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    ConfirmPasswordChanged(String),
    RegisterSubmitted,
    RegisterCancelled,
    RegisterFetched(fetch::Result<LoggedUser>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    log!(model.form);
    match msg {
        Msg::FirstNameChanged(first_name) => model.form.first_name = first_name,
        Msg::LastNameChanged(last_name) => model.form.last_name = last_name,
        Msg::EmailChanged(email) => model.form.email = email,
        Msg::PhoneChanged(phone) => model.form.phone = phone,
        Msg::PasswordChanged(password) => model.form.password = password,
        Msg::ConfirmPasswordChanged(confirm_password) => model.form.confirm_password = confirm_password,
        Msg::RegisterCancelled => route::go_to(route::Route::Home, orders),
        Msg::RegisterSubmitted => {
            if model.form.confirm_password == model.form.password {
                let request = Request::new("/v1/users/add")
                    .method(Method::Post)
                    .json::<shared::RegisterRequestBody>(&model.form.clone().into());
                orders.perform_cmd(async { Msg::RegisterFetched(async { request?.fetch().await?.check_status()?.json().await }.await) });
            }
        }
        Msg::RegisterFetched(Ok(logged_user)) => {
            LocalStorage::insert(crate::STORAGE_KEY, &logged_user).expect("save user");
            //orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(logged_user)));
            route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::RegisterFetched(Err(error)) => log!(error),
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(
        "Register",
        div![
            class!["col-md-4", "offset-md-4" "mt-4"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Register",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["First Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.form.first_name, },
                                input_ev(Ev::Input, Msg::FirstNameChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Last Name"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.form.last_name, },
                                input_ev(Ev::Input, Msg::LastNameChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Email"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.form.email, },
                                input_ev(Ev::Input, Msg::EmailChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Phone"],
                            input![
                                attrs! { At::Type => "text", At::Class => "form-control", At::Value => model.form.phone, },
                                input_ev(Ev::Input, Msg::PhoneChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label!["Password"],
                            input![
                                attrs! { At::Type => "password", At::Class => "form-control", At::Value => model.form.password, },
                                input_ev(Ev::Input, Msg::PasswordChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            label![
                                class!["control-label", "text-danger" => model.form.confirm_password != model.form.password ],
                                attrs! { At::For => "confirm_password_error" },
                                "Confirm Password"
                            ],
                            input![
                                class!["form-control", "is-invalid" => model.form.confirm_password != model.form.password ],
                                attrs! { At::Type => "password", At::Id => "confirm_password_error", At::Value => model.form.confirm_password, },
                                input_ev(Ev::Input, Msg::ConfirmPasswordChanged),
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
                            a![
                                class!["btn", "btn-primary", "text-white", "ml-2"],
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
