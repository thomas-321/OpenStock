use crate::Context;
use crate::{Message, WindowMessage};
use iced::Task;

pub trait Window {
    fn update(&mut self, message: WindowMessage, tab_id: TabId, app: Context) -> Task<Message>;
    //) -> (Option<Box<dyn Window>>, Task<WindowMessage>);
    fn view(&self, tab_id: TabId) -> iced::Element<'_, Message>;
    fn get_title(&self) -> &str;
}

#[derive(Default)]
pub struct IdGenerator {
    next_pane_id: u32,
    next_tab_id: u32,
}

impl IdGenerator {
    pub fn get_new_pane_id(&mut self) -> PaneId {
        self.next_pane_id += 1;
        PaneId {
            id: self.next_pane_id - 1,
        }
    }
    pub fn get_new_tab_id(&mut self) -> TabId {
        self.next_tab_id += 1;
        TabId {
            id: self.next_tab_id - 1,
        }
    }
}

/// TabId indicates the id of one window tab
#[derive(Clone, Copy, PartialEq)]
pub struct TabId {
    pub id: u32,
}

/// PaneId indicates which pane a window is a part of
#[derive(Clone, PartialEq)]
pub struct PaneId {
    pub id: u32,
}

/// Represent all standard data each window has
pub struct Tab {
    pub pane_id: PaneId,
    pub tab_id: TabId,
    pub tab_title: String,
    pub window: Box<dyn Window>,
}

impl Tab {
    pub fn new<T: Window + 'static>(pane_id: PaneId, tab_id: TabId, window: T) -> Self {
        Self {
            pane_id,
            tab_id,
            tab_title: window.get_title().to_string(),
            window: Box::new(window),
        }
    }
}

pub struct Pane {
    pub pane_id: PaneId,
    pub active_tab_id: Option<TabId>,
}
