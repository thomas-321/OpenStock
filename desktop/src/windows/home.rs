use std::sync::Arc;

use iced::widget::{center, center_x, center_y, column, container, row, scrollable, text};
use iced::{Center, Fill, Task};

use crate::util::ApiClient;
use crate::widgets::sidebar::get_main_sidebar;
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

        // content
        let welcome_message = format!("Home Page, Welcome {}", tex);
        //center(container(text(format!("Home Page, Welcome {}", tex)))).into()

        let content = center(container(
            scrollable(
                column![
                    text(welcome_message),
                    row!["tbd", "tbd"].spacing(10).align_y(Center).wrap(),
                    "end"
                ]
                .width(Fill),
            )
            .height(Fill),
        ));

        column![content.padding(5).height(Fill)].height(Fill).into()
    }

    fn get_sidebar(&self, ctx: &Context) -> Option<iced::Element<'_, Message>> {
        ctx.role.as_ref().map(|role| get_main_sidebar(role))
    }

    fn get_title(&self) -> &str {
        "Home"
    }
}

impl HomeWindow {}
