use iced::widget::{button, center, column, container, text, text_input};
use iced::widget::Space;
use iced::Length;

use crate::error::Error;
use crate::pages::Page;
use crate::Message;

#[derive(Clone)]
pub enum LoginPageMessage {
    LoginPressed,
    RegisterPressed,
    TextFieldChanged(Field, String),
}

#[derive(Default)]
pub struct LoginPage {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Clone)]
pub enum Field {
    Username,
    Password,
}

impl Page for LoginPage {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::Login(msg) = message {
            match msg {
                LoginPageMessage::TextFieldChanged(field, value) => {
                    let value = if value.is_empty() { None } else { Some(value) };

                    match field {
                        Field::Username => self.username = value,
                        Field::Password => self.password = value,
                    }
                }
                LoginPageMessage::LoginPressed => {
                    if self.username.is_none() || self.password.is_none() {
                        // show message that username or password should be filled in
                        println!("Username or password is not filled in");
                    };

                    match try_login(self.username.clone()?, self.password.clone()?) {
                        Ok(value) => {
                            // store apiKey
                            println!("Succesful login, received api-key: {}", value);
                        }
                        Err(e) => {
                            // handle error
                            println!("error: {}", e.to_string());
                        }
                    };
                    println!("login presseda");
                }
                LoginPageMessage::RegisterPressed => {
                    println!("register pressed");
                }
            }
        }
        None
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let container = container(
            column![
                text("Username:").size(10),
                text_input("username", self.username.as_deref().unwrap_or(""))
                    .padding(10)
                    .on_input(|s| {
                        Message::Login(LoginPageMessage::TextFieldChanged(Field::Username, s))
                    }),
                Space::new().height(20),
                text("Password:").size(10),
                text_input("password", self.password.as_deref().unwrap_or(""))
                    .secure(true)
                    .width(Length::Fill)
                    .padding(10)
                    .on_input(|s| Message::Login(LoginPageMessage::TextFieldChanged(
                        Field::Password,
                        s
                    ))),
                Space::new().height(20),
                button("Login")
                    .width(Length::Fill)
                    .on_press(Message::Login(LoginPageMessage::LoginPressed)),
                Space::new().height(40),
                text("Click here to register:").size(10),
                Space::new().height(5),
                button("Register") // Todo: Change this to a hyprlink instead of a button
                    .width(Length::Fill)
                    .on_press(Message::Login(LoginPageMessage::RegisterPressed)),
            ]
            .width(500),
        );

        center(container).into()
    }
}

/// Sends a login request
fn try_login(username: String, password: String) -> Result<String, Error> {
    todo!();
}