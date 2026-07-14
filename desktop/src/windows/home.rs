use std::sync::Arc;

use iced::widget::{center, center_x, center_y, column, container, row, scrollable, text};
use iced::{Center, Fill, Task};

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

        let sidebar = container(
            column![center_x("Openstock v0.1"), "Articles", "Users"]
                .spacing(10)
                .padding(2)
                .width(200),
        )
        .height(Fill)
        .style(container::secondary);

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

        column![row![sidebar, content].padding(5).height(Fill)]
            .height(Fill)
            .into()
    }
    fn get_title(&self) -> &str {
        "Home page"
    }
}

impl HomeWindow {}
