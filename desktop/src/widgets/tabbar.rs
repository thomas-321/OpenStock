use iced::widget::{button, row, text};

use crate::{windows::window::TabId, Message};

pub fn create_tabbar(
    activetab: TabId,
    pane: Vec<(TabId, String)>,
) -> iced::Element<'static, Message> {
    let mut tabbar = row![];
    println!("tabbar test input: {:?}", pane);

    for (tab_id, title) in pane {
        println!(
            "Created tab with tab_id: {}, and tab name: {}",
            tab_id.id, title
        );
        tabbar = tabbar.push(
            button(text(title))
                .width(100)
                .on_press(Message::GlobalStateMessage(
                    TabId { id: u32::MAX },
                    crate::StateMessage::FocusTab(tab_id),
                ))
                .style(if tab_id == activetab {
                    button::primary
                } else {
                    button::secondary
                }),
        )
    }
    tabbar.into()
}
