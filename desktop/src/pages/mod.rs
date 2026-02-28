pub mod home;
pub mod login;

use crate::Message;
pub trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<'_, Message>;
}

pub use login::LoginPage;