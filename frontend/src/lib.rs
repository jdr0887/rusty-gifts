#![allow(clippy::must_use_candidate)]

use seed::prelude::*;
use seed::*;
use serde::{Deserialize, Serialize};

const LOGIN: &str = "login";
const REGISTER: &str = "register";
const API_URL: &str = "http://127.0.0.1:8080";
const STORAGE_KEY: &str = "seed_auth_example";

struct Model {
    email: String,
    password: String,
    base_url: Url,
    page: Page,
    secret_message: Option<String>,
    user: Option<LoggedUser>,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: i32,
    email: String,
    password: String,
    token: String,
}

enum Page {
    Home,
    Login,
    Register,
    NotFound,
}

impl Page {
    fn init(mut url: Url, user: Option<&LoggedUser>, orders: &mut impl Orders<Msg>) -> Self {
        let next_path_part = url.next_path_part();
        log!("next_path_part: {}", next_path_part);
        match next_path_part {
            Some(LOGIN) => Self::Login,
            Some(REGISTER) => Self::Register,
            None => {
                log!("next_path_part: {}", next_path_part);
                if let Some(user) = user {
                    send_request_to_top_secret(user.token.clone(), orders)
                };
                if user.is_none() {
                    Self::Login
                } else {
                    Self::Home
                }
            }
            _ => Self::NotFound,
        }
    }
}

fn send_request_to_top_secret(token: String, orders: &mut impl Orders<Msg>) {
    orders.perform_cmd(async {
        Msg::TopSecretFetched(
            async {
                Request::new(format!("{}/top_secret", API_URL))
                    .header(Header::bearer(token))
                    .fetch()
                    .await?
                    .check_status()?
                    .text()
                    .await
            }
            .await,
        )
    });
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    let user = LocalStorage::get(STORAGE_KEY).ok();
    Model {
        // email: "john@example.com".to_owned(),
        // password: "1234".to_owned(),
        email: "".to_owned(),
        password: "".to_owned(),
        base_url: url.to_base_url(),
        page: Page::init(url, user.as_ref(), orders),
        secret_message: None,
        user,
    }
}

struct_urls!();

impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }
    pub fn register(self) -> Url {
        self.base_url().add_path_part(REGISTER)
    }
}

enum Msg {
    UrlChanged(subs::UrlChanged),
    EmailChanged(String),
    // FirstNameChanged(String),
    // LastNameChanged(String),
    PasswordChanged(String),
    LoginClicked,
    LoginFetched(fetch::Result<LoggedUser>),
    TopSecretFetched(fetch::Result<String>),
    LogoutClicked,
    RegisterClicked,
    RegisterCancelClicked,
}

#[derive(Serialize)]
struct LoginRequestBody<'a> {
    email: &'a str,
    password: &'a str,
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![match &model.page {
        Page::Home => home_page(&model),
        Page::Login => login_page(&model),
        Page::Register => register_page(&model),
        Page::NotFound => div!["404"],
    }]
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url, model.user.as_ref(), orders);
        }
        Msg::EmailChanged(email) => model.email = email,
        // Msg::FirstNameChanged(first_name) => model.first_name = Some(first_name),
        // Msg::LastNameChanged(last_name) => model.last_name = Some(last_name),
        Msg::PasswordChanged(password) => model.password = password,
        Msg::LoginClicked => {
            let request = Request::new(format!("{}/users/login", API_URL)).method(Method::Post).json(&LoginRequestBody {
                email: &model.email,
                password: &model.password,
            });
            orders.perform_cmd(async { Msg::LoginFetched(async { request?.fetch().await?.check_status()?.json().await }.await) });
        }
        Msg::LoginFetched(Ok(logged_user)) => {
            LocalStorage::insert(STORAGE_KEY, &logged_user).expect("save user");
            model.user = Some(logged_user);
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).home()));
        }
        Msg::TopSecretFetched(Ok(secret_message)) => {
            model.secret_message = Some(secret_message);
        }
        Msg::LoginFetched(Err(error)) | Msg::TopSecretFetched(Err(error)) => log!(error),
        Msg::LogoutClicked => {
            LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
            model.user = None;
            model.secret_message = None;
        }
        Msg::RegisterClicked => {
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).register()));
        }
        Msg::RegisterCancelClicked => {
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).login()));
        }
    }
}

fn home_page(model: &Model) -> Node<Msg> {
    div![
        ul![
            li![a![attrs! { At::Href => Urls::new(&model.base_url).home() }, "Home",]],
            if model.user.as_ref().is_none() {
                li![a![attrs! { At::Href => Urls::new(&model.base_url).login() }, "Login",]]
            } else {
                li![a![attrs! { At::Href => "" }, "Logout", ev(Ev::Click, |_| Msg::LogoutClicked),]]
            }
        ],
        format!("Welcome home {}!", model.user.as_ref().map(|user| user.email.to_owned()).unwrap_or_default()),
        div![&model.secret_message],
    ]
}

fn register_page(model: &Model) -> Node<Msg> {
    div![
        class!["col-md-6", "offset-md-3" "mt-5"],
        div![
            attrs! { At::Class => "card" },
            h4![attrs! { At::Class => "card-header" }, "Gift App Register",],
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
                            "Login",
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::LoginClicked
                            })
                        ],
                        button![
                            class!["btn", "btn-primary", "ml-2"],
                            "Cancel",
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::RegisterCancelClicked
                            })
                        ],
                    ],
                ],
            ],
        ],
    ]
}

fn login_page(model: &Model) -> Node<Msg> {
    div![
        class!["col-md-6", "offset-md-3" "mt-5"],
        div![
            attrs! { At::Class => "card" },
            h4![attrs! { At::Class => "card-header" }, "Gift App Login",],
            div![
                attrs! { At::Class => "card-body" },
                form![
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
                        label!["Password"],
                        input![
                            attrs! { At::Type => "password", At::Class => "form-control", At::Value => model.password, },
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
                                Msg::LoginClicked
                            })
                        ],
                        a![
                            class!["btn", "btn-link"],
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
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
