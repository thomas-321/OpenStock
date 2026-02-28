use iced::widget::Button;
use iced::widget::{button, text};
use iced::{Element, Theme};

use crate::pages::home::HomePageMessage;
use crate::pages::login::{LoginPage, LoginPageMessage};
use crate::pages::Page;

mod error;
mod pages;

fn main() -> iced::Result {
    iced::application(Openstock::default, Openstock::update, Openstock::view)
        .title(Openstock::title)
        .centered()
        .run()
}

#[derive(Clone)]
enum Message {
    Login(LoginPageMessage),
    Home(HomePageMessage),
}

pub struct Openstock {
    Page: Box<dyn Page>,
    theme: Theme,
}

fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
    button(text(label)).padding([12, 24])
}

impl Openstock {
    fn title(&self) -> String {
        "Openstock".to_string()
    }

    fn update(&mut self, message: Message) {
        if let Some(new_Page) = self.Page.update(message) {
            self.Page = new_Page;
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.Page.view()
    }
}

impl Default for Openstock {
    fn default() -> Self {
        Self {
            Page: Box::new(LoginPage::default()),
            theme: Theme::Dark,
        }
    }
}