use iced::widget::{button, text};
use iced::widget::{center, container, Button};
use iced::{Element, Task, Theme};
use models::auth::LoginResponse;

use crate::error::AppError;
use crate::state::ApiClient;
use crate::windows::home::HomeWindowMessage;
use crate::windows::login::LoginWindowMessage;
use crate::windows::window::{IdGenerator, Pane, Tab, TabId, Window};

mod error;
mod services;
mod state;
mod widgets;
mod windows;

fn main() -> iced::Result {
    iced::application(Openstock::default, Openstock::update, Openstock::view)
        .title(Openstock::title)
        .centered()
        .run()
}

#[derive(Clone)]
enum Message {
    Tab(TabId, WindowMessage),
    GlobalStateMessage(StateMessage),
}

#[derive(Clone)]
enum StateMessage {
    LoginFinshed(Result<LoginResponse, AppError>),
}

#[derive(Clone)]
enum WindowMessage {
    Login(LoginWindowMessage),
    Home(HomeWindowMessage),
}

#[derive(Clone)]
pub struct Context {
    api: ApiClient,
}

pub struct Openstock {
    panes: Vec<Pane>,
    tabs: Vec<Tab>,
    theme: Theme,
    id_generator: IdGenerator,
    context: Context,
}

impl Openstock {
    fn title(&self) -> String {
        "Openstock".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tab(tab_id, w_message) => {
                match self.tabs.iter_mut().find(|tab| tab.tab_id == tab_id) {
                    Some(tab) => tab.window.update(w_message, self.context.clone()),
                    None => {
                        println!("Received message from destroyed window");
                        Task::none()
                    }
                }
            }
            Message::GlobalStateMessage(smessage) => self.handle_state_message(smessage),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.tabs.first() {
            Some(tab) => tab.window.view(tab.tab_id),
            None => center(container(text("Error loading page"))).into(),
        }
    }

    fn handle_state_message(&mut self, message: StateMessage) -> Task<Message> {
        match message {
            StateMessage::LoginFinshed(result) => {
                match result {
                    Ok(value) => println!("Ok: {}", value.token),
                    Err(e) => println!("Error: {}", e),
                }
                Task::none()
            }
        }
    }
}

impl Default for Openstock {
    fn default() -> Self {
        let mut id_generator = IdGenerator::default();

        let pane = windows::window::Pane {
            pane_id: id_generator.get_new_pane_id(),
            active_tab_id: None,
        };
        let default_window = windows::LoginWindow::default();
        let tab = windows::window::Tab::new(
            id_generator.get_new_pane_id(),
            id_generator.get_new_tab_id(),
            default_window,
        );

        Self {
            theme: Theme::Dark,
            id_generator: IdGenerator::default(),
            panes: vec![pane],
            tabs: vec![tab],
            context: Context {
                api: ApiClient::new("http://localhost:8080".to_string()),
            },
        }
    }
}
