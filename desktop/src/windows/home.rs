use std::sync::Arc;

use iced::widget::{center, container, text};
use iced::Task;

use crate::util::ApiClient;
use crate::windows::window::{TabId, Window};
use crate::{Context, Message, WindowMessage};

#[derive(Clone)]
pub enum HomeWindowMessage {}

#[derive(Default)]
pub struct HomeWindow {}

impl Window for HomeWindow {
    fn update(
        &mut self,
        _message: WindowMessage,
        _tab_id: TabId,
        _app: Arc<ApiClient>,
    ) -> Task<Message> {
        todo!()
    }

    fn view(&self, _tab_id: TabId, context: &Context) -> iced::Element<'_, Message> {
        let tex = match &context.user {
            Some(user) => &user.first_name,
            None => "UNKNOWN",
        };
        center(container(text(format!("Home Page, Welcome {}", tex)))).into()
    }
    fn get_title(&self) -> &str {
        "Home page"
    }
}
