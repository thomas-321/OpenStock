use std::sync::Arc;

use iced::widget::Space;
use iced::widget::{button, center, column, container, text, text_input};
use iced::Length;
use iced::Task;
use models::auth::LoginResponse;

use crate::error::AppError;
use crate::services::auth_service;
use crate::util::ApiClient;
use crate::windows::window::{TabId, Window};
use crate::{Context, Message, StateMessage, WindowMessage};

#[derive(Clone)]
pub enum LoginWindowMessage {
    LoginPressed,
    RegisterPressed,
    DevLoginPressed,
    TextFieldChanged(Field, String),
    LoginFailed(AppError),
}

#[derive(Default)]
pub struct LoginWindow {
    window_title: String,
    email: Option<String>,
    password: Option<String>,
    error: Option<AppError>,
}

#[derive(Clone)]
pub enum Field {
    Email,
    Password,
}

impl Window for LoginWindow {
    fn update(
        &mut self,
        message: WindowMessage,
        tab_id: TabId,
        api: Arc<ApiClient>,
    ) -> Task<Message> {
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
            LoginWindowMessage::DevLoginPressed => Task::perform(
                auth_service::login(
                    api,
                    Some("jan.vandermeer@example.com".to_string()),
                    Some("pass001".to_string()),
                ),
                move |result| {
                    Message::GlobalStateMessage(tab_id, StateMessage::LoginFinshed(result))
                },
            ),
            LoginWindowMessage::LoginPressed => {
                println!("login pressed");

                let email = self.email.clone();
                let password = self.password.clone();
                let api = api.clone();

                Task::perform(auth_service::login(api, email, password), move |result| {
                    Message::GlobalStateMessage(tab_id, StateMessage::LoginFinshed(result))
                })
            }
            LoginWindowMessage::RegisterPressed => {
                println!("register pressed");
                Task::none()
            }
            LoginWindowMessage::LoginFailed(error) => {
                println!("login failed with error: {}", error);
                match error {
                    AppError::InvalidLogin => {
                        self.email = None;
                        self.password = None;
                    }
                    _ => self.error = Some(error),
                }
                Task::none()
            }
        }
    }

    fn view(&self, tab_id: TabId, _context: &Context) -> iced::Element<'_, Message> {
        let container = container(
            column![
                text("Email:").size(10),
                text_input("email", self.email.as_deref().unwrap_or(""))
                    .padding(10)
                    .on_input(move |s| {
                        create_message(
                            tab_id,
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
                            tab_id,
                            LoginWindowMessage::TextFieldChanged(Field::Password, s),
                        )
                    }),
                Space::new().height(20),
                button("Login")
                    .style(button::primary)
                    .width(Length::Fill)
                    .on_press(create_message(tab_id, LoginWindowMessage::LoginPressed)),
                Space::new().height(10),
                button("Dev quick login")
                    .style(button::primary)
                    .width(Length::Fill)
                    .on_press(create_message(tab_id, LoginWindowMessage::DevLoginPressed)),
                Space::new().height(40),
                text("Click here to register:").size(10),
                Space::new().height(5),
                button("Register") // TODO: Change this to a hyprlink instead of a button
                    .style(button::secondary)
                    .width(Length::Fill)
                    .on_press(create_message(tab_id, LoginWindowMessage::RegisterPressed)),
            ]
            .width(500),
        );

        center(container).into()
    }

    /// The login window does not have a sidebar
    fn get_sidebar(&self, _context: &Context) -> Option<iced::Element<'_, Message>> {
        None
    }

    fn get_title(&self) -> &str {
        "Login"
    }
}

fn create_message(tab_id: TabId, message: LoginWindowMessage) -> Message {
    Message::Tab(tab_id, WindowMessage::Login(message))
}
