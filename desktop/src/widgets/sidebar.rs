use iced::widget::{center_x, column, container};
use iced::Fill;
use models::user::Role;

use crate::Message;

/// The sidebar allows navigation inside the app.
/// Uses the role input to only fill the options the user has rights to
pub fn get_main_sidebar(_role: &Role) -> iced::Element<'static, Message> {
    container(
        column![center_x("Openstock v0.1"), "Articles", "Users"]
            .spacing(10)
            .padding(2)
            .width(200),
    )
    .height(Fill)
    .style(container::secondary)
    .into()
}
