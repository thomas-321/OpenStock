use iced::Task;

use crate::windows::window::{TabId, Window};
use crate::{Context, Message, WindowMessage};

#[derive(Clone)]
pub enum HomeWindowMessage {}

#[derive(Default)]
pub struct HomeWindow {}

impl Window for HomeWindow {
    fn update(&mut self, _message: WindowMessage, _app: Context) -> Task<Message> {
        todo!()
    }

    fn view(&self, _tab_id: TabId) -> iced::Element<'_, Message> {
        todo!()
    }
    fn get_title(&self) -> &str {
        "Home page"
    }
}
