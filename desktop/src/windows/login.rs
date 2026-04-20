use iced::widget::Space;
use iced::widget::{button, center, column, container, text, text_input};
use iced::Length;
use iced::Task;
use models::auth::LoginResponse;

use crate::error::AppError;
use crate::services::auth_service;
use crate::windows::window::{TabId, Window};
use crate::{Context, Message, StateMessage, WindowMessage};

#[derive(Clone)]
pub enum LoginWindowMessage {
    LoginPressed,
    RegisterPressed,
    TextFieldChanged(Field, String),
    LoginFinished(Result<LoginResponse, AppError>),
}

#[derive(Default)]
pub struct LoginWindow {
    window_title: String,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Clone)]
pub enum Field {
    Email,
    Password,
}

impl Window for LoginWindow {
    fn update(&mut self, message: WindowMessage, context: Context) -> Task<Message> {
        let WindowMessage::Login(msg) = message else {
            return Task::none();
        };

        match msg {
            LoginWindowMessage::TextFieldChanged(field, value) => {
                let value = if value.is_empty() { None } else { Some(value) };

                match field {
                    Field::Email => self.email = value,
                    Field::Password => self.password = value,
                }
                Task::none()
            }
            LoginWindowMessage::LoginPressed => {
                println!("login pressed");

                let email = self.email.clone();
                let password = self.password.clone();
                let api = context.api.clone();

                //Task::none(),
                Task::perform(auth_service::login(api, email, password), |result| {
                    Message::GlobalStateMessage(StateMessage::LoginFinshed(result))
                })
            }
            LoginWindowMessage::RegisterPressed => {
                println!("register pressed");
                Task::none()
            }
            LoginWindowMessage::LoginFinished(result) => {
                println!("login finshed");
                Task::none()
            }
        }
    }

    fn view(&self, tab_id: TabId) -> iced::Element<'_, Message> {
        let tab_id = tab_id.clone();
        let container = container(
            column![
                text("Username:").size(10),
                text_input("username", self.email.as_deref().unwrap_or(""))
                    .padding(10)
                    .on_input(move |s| {
                        create_message(
                            tab_id.clone(),
                            LoginWindowMessage::TextFieldChanged(Field::Email, s),
                        )
                    }),
                Space::new().height(20),
                text("Password:").size(10),
                text_input("password", self.password.as_deref().unwrap_or(""))
                    .secure(true)
                    .width(Length::Fill)
                    .padding(10)
                    .on_input(move |s| {
                        create_message(
                            tab_id.clone(),
                            LoginWindowMessage::TextFieldChanged(Field::Password, s),
                        )
                    }),
                Space::new().height(20),
                button("Login").width(Length::Fill).on_press(create_message(
                    tab_id.clone(),
                    LoginWindowMessage::LoginPressed
                )),
                Space::new().height(40),
                text("Click here to register:").size(10),
                Space::new().height(5),
                button("Register") // Todo: Change this to a hyprlink instead of a button
                    .width(Length::Fill)
                    .on_press(create_message(
                        tab_id.clone(),
                        LoginWindowMessage::RegisterPressed
                    )),
            ]
            .width(500),
        );

        center(container).into()
    }

    fn get_title(&self) -> &str {
        "Login page"
    }
}

//impl LoginWindow {
fn create_message(tab_id: TabId, message: LoginWindowMessage) -> Message {
    Message::Tab(tab_id, WindowMessage::Login(message))
}
//}
