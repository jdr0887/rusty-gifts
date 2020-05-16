use crate::page::ViewPage;
use crate::route;
use crate::session::Session;
use crate::GMsg;
use crate::LoggedUser;
use seed::prelude::*;
use seed::*;

#[derive(Default, Clone, Debug)]
pub struct Form {
    email: String,
    password: String,
}

impl From<Form> for shared::LoginRequestBody {
    fn from(form: Form) -> shared::LoginRequestBody {
        shared::LoginRequestBody {
            email: form.email.clone(),
            password: form.password.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    session: Session,
    form: Form,
    secret_message: Option<String>,
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
            // log!("login.sink.session.viewer(): ", session.viewer());
            model.session = session;
            route::go_to(route::Route::GiftIdeas, orders);
        }
        _ => (),
    }
}

#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    LoginSubmitted,
    LoginCancelled,
    LoginFetched(fetch::Result<LoggedUser>),
    RegisterClicked,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    log!("msg: ", msg);
    match msg {
        Msg::EmailChanged(email) => model.form.email = email,
        Msg::PasswordChanged(password) => model.form.password = password,
        Msg::LoginSubmitted => {
            let request = Request::new("/v1/users/login")
                .method(Method::Post)
                .json::<shared::LoginRequestBody>(&model.form.clone().into());
            orders.perform_cmd(async { Msg::LoginFetched(async { request?.fetch().await?.check_status()?.json().await }.await) });
        }
        Msg::LoginCancelled => route::go_to(route::Route::Home, orders),
        Msg::LoginFetched(Ok(logged_user)) => {
            log!("LoginFetched: ", logged_user);
            LocalStorage::insert(crate::STORAGE_KEY, &logged_user).expect("save user");
            orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(logged_user)));
            // route::go_to(route::Route::GiftIdeas, orders);
        }
        Msg::LoginFetched(Err(error)) => log!(error),
        Msg::RegisterClicked => route::go_to(route::Route::Register, orders),
        // Msg::FormSubmitted => match model.form.trim_fields().validate() {
        //     Ok(valid_form) => {
        //         model.problems.clear();
        //         orders.perform_cmd(request::login::login(&valid_form, Msg::LoginCompleted));
        //     }
        //     Err(problems) => {
        //         model.problems = problems;
        //     }
        // },
        // Msg::FieldChanged(field) => {
        //     model.form.upsert_field(field);
        // }
        // Msg::LoginCompleted(Ok(viewer)) => {
        //     viewer.store();
        //     orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(viewer)));
        // }
        // Msg::LoginCompleted(Err(problems)) => {
        //     model.problems = problems;
        // }
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(
        "Login",
        div![
            class!["col-md-4", "offset-md-4" "mt-4"],
            div![
                attrs! { At::Class => "card" },
                h4![attrs! { At::Class => "card-header" }, "Login",],
                div![
                    attrs! { At::Class => "card-body" },
                    form![
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
                            label![class!["control-label"], attrs! { At::For => "password_error" }, "Password"],
                            input![
                                class!["form-control"],
                                attrs! { At::Id => "password_error", At::Type => "password", At::Value => model.form.password, },
                                input_ev(Ev::Input, Msg::PasswordChanged)
                            ],
                        ],
                        div![
                            attrs! { At::Class => "form-group" },
                            button![
                                class!["btn", "btn-primary"],
                                "Login",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::LoginSubmitted
                                })
                            ],
                            a![
                                class!["btn", "btn-primary", "text-white", "ml-1"],
                                "Cancel",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::LoginCancelled
                                })
                            ],
                            a![
                                class!["btn", "btn-primary", "text-white", "ml-1", "float-right"],
                                "Register",
                                ev(Ev::Click, |event| {
                                    event.prevent_default();
                                    Msg::RegisterClicked
                                })
                            ],
                        ],
                    ],
                ],
            ],
        ],
    )
}
