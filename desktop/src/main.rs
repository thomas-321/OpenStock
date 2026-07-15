use iced::widget::{center, column, container, text};
use iced::{color, Element, Task, Theme};
use std::sync::Arc;

use models::auth::LoginResponse;
use models::user::{Role, User};

use crate::error::AppError;
use crate::services::user_service;
use crate::util::ApiClient;
use crate::widgets::tabbar::create_tabbar;
use crate::windows::home::{HomeWindow, HomeWindowMessage};
use crate::windows::login::{LoginWindow, LoginWindowMessage};
use crate::windows::window::{IdGenerator, Pane, PaneId, Tab, TabId};

mod error;
mod services;
mod util;
mod widgets;
mod windows;

fn main() -> iced::Result {
    iced::application(Openstock::default, Openstock::update, Openstock::view)
        .title(Openstock::title)
        .theme(Openstock::theme)
        .centered()
        .run()
}

#[derive(Clone)]
enum Message {
    Tab(TabId, WindowMessage),
    GlobalStateMessage(TabId, StateMessage),
}

#[derive(Clone)]
enum StateMessage {
    CreateTab(CreateTab, TabId),
    RemoveTab(TabId),
    FocusTab(TabId),
    LoginFinshed(Result<LoginResponse, AppError>),
    InitFinished(Result<(User, Role), AppError>),
}

#[derive(Clone)]
enum CreateTab {
    HomeWindow,
    Login,
}

#[derive(Clone)]
enum WindowMessage {
    Login(LoginWindowMessage),
    Home(HomeWindowMessage),
}

#[derive(Clone)]
pub struct Context {
    api: Arc<ApiClient>,
    user: Option<User>,
    role: Option<Role>,
    theme: Theme,
}

pub struct Openstock {
    panes: Vec<Pane>,
    tabs: Vec<Tab>,
    id_generator: IdGenerator,
    context: Arc<Context>,
}

impl Openstock {
    fn title(&self) -> String {
        "Openstock".to_string()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tab(tab_id, w_message) => {
                match self.tabs.iter_mut().find(|tab| tab.tab_id == tab_id) {
                    Some(tab) => tab
                        .window
                        .update(w_message, tab_id, self.context.api.clone()),
                    None => {
                        println!("Received message from destroyed window");
                        Task::none()
                    }
                }
            }
            Message::GlobalStateMessage(tab_id, smessage) => {
                self.handle_state_message(tab_id, smessage)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let context: &Context = self.context.as_ref();

        let focused_tab_id = self.panes.first().expect("").active_tab_id.expect("");
        let active_tab = self
            .tabs
            .iter()
            .find(|tab| tab.tab_id == focused_tab_id)
            .expect("");

        let sidebar = active_tab.window.get_sidebar(context);
        let tabbar: iced::Element<'_, Message> = create_tabbar(
            focused_tab_id,
            self.get_tab_ids_and_names_of_pane(
                self.panes
                    .first()
                    .expect("There should be at least one pane.")
                    .pane_id
                    .clone(),
            ),
        );

        let mut view = iced::widget::row![];

        if let Some(s) = sidebar {
            view = view.push(s);
        }

        if active_tab.tab_title == "Login".to_string() {
            view = view.push(column![self.create_content_window(context)]);
        } else {
            view = view.push(column![tabbar, self.create_content_window(context)]);
        }
        Element::from(view).explain(color!(0x0000ff))
    }

    /// for now only with one pane
    fn create_content_window(&self, context: &Context) -> iced::Element<'_, Message> {
        let active_tab_first_pane = self
            .panes
            .first()
            .expect("there should be a pane")
            .active_tab_id
            .expect("there should be an active tab");

        let tab = self
            .tabs
            .iter()
            .find(|tab| tab.tab_id == active_tab_first_pane)
            .expect("asaa");

        tab.window.view(tab.tab_id, context)
        //todo!()
    }

    fn theme(&self) -> Option<Theme> {
        Some(self.context.theme.clone())
    }

    /// returns the pane_id of the pane to which the tab belongs
    fn get_pane_id_from_tab_id(&mut self, tab_id: &TabId) -> PaneId {
        let o_pane_id = self
            .tabs
            .iter()
            .find(|&tab| tab.tab_id == *tab_id)
            .map(|t| t.pane_id.clone());

        match o_pane_id {
            Some(pane_id) => pane_id,
            None => {
                if self.panes.is_empty() {
                    self.create_pane();
                }
                self.panes.first().unwrap().pane_id.clone()
            }
        }
    }

    fn handle_state_message(
        &mut self,
        source_tab_id: TabId,
        message: StateMessage,
    ) -> Task<Message> {
        match message {
            StateMessage::LoginFinshed(result) => {
                match result {
                    Ok(value) => {
                        self.context.api.set_token(value.token);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        return Task::done(Message::Tab(
                            source_tab_id,
                            WindowMessage::Login(LoginWindowMessage::LoginFailed(e)),
                        ));
                    }
                }

                let api = self.context.api.clone();
                Task::perform(
                    async {
                        let (user, role) = tokio::join!(
                            user_service::get_current_user_data(api.clone()),
                            user_service::get_current_user_role(api)
                        );

                        let user = user?;
                        let role = role?;

                        Ok((user, role))
                    },
                    move |result| {
                        Message::GlobalStateMessage(
                            source_tab_id,
                            StateMessage::InitFinished(result),
                        )
                    },
                )
            }
            StateMessage::InitFinished(result) => match result {
                Ok((user, role)) => {
                    let mut context = (*self.context).clone();
                    context.user = Some(user);
                    context.role = Some(role);
                    self.context = Arc::new(context);
                    let new_tab = self.create_tab(CreateTab::HomeWindow, &source_tab_id);
                    self.set_active_tab(&new_tab);
                    self.remove_tab(source_tab_id);
                    Task::none()
                }
                Err(e) => Task::done(Message::Tab(
                    source_tab_id,
                    WindowMessage::Login(LoginWindowMessage::LoginFailed(e)),
                )),
            },
            StateMessage::CreateTab(tab_type, source_tab_id) => {
                self.create_tab(tab_type, &source_tab_id);
                Task::none()
                //Task::done(Message::GlobalStateMessage(
                //    tab_id, // not used
                //    StateMessage::RemoveTab(tab_id),
                //))
            }
            StateMessage::RemoveTab(tab_id) => {
                self.remove_tab(tab_id);
                Task::none()
            }
            StateMessage::FocusTab(tab_id) => {
                self.set_active_tab(&tab_id);
                Task::none()
            }
        }
    }

    /// Create a new tab and returns its TabId
    fn create_tab(&mut self, tab_type: CreateTab, source_tab_id: &TabId) -> TabId {
        let pane_id = self.get_pane_id_from_tab_id(source_tab_id);
        self.create_tab_in_pane(tab_type, &pane_id)
    }

    /// Create a new tab in the given pane
    fn create_tab_in_pane(&mut self, tab_type: CreateTab, pane_id: &PaneId) -> TabId {
        let new_tab_id = self.id_generator.get_new_tab_id();
        self.tabs.push(Tab::new(
            pane_id.clone(),
            new_tab_id,
            match tab_type {
                CreateTab::HomeWindow => Box::new(HomeWindow {}),
                CreateTab::Login => Box::new(LoginWindow::default()),
            },
        ));

        if let Some(pane) = self
            .panes
            .iter_mut()
            .find(|pane| pane.pane_id == *pane_id && pane.active_tab_id.is_none())
        {
            pane.active_tab_id = Some(new_tab_id);
        } else {
            println!("There should exists a pane");
        }
        new_tab_id
    }

    fn remove_tab(&mut self, tab_id: TabId) {
        let Some(index) = self.tabs.iter().position(|tab| tab.tab_id == tab_id) else {
            println!("Tried removing tab, id not found");
            return;
        };

        // If the last window is closed the home window is reopend
        if self.tabs.len() == 1 {
            let new_tab_id = self.create_tab(CreateTab::HomeWindow, &tab_id);
            let pane_id = self.get_pane_id_from_tab_id(&tab_id);
        }

        self.tabs.remove(index);
        // ...
    }

    /// creates a new pane and sets the given tab as its active_tab_id
    fn create_pane(&mut self) -> PaneId {
        let new_pane_id = self.id_generator.get_new_pane_id();
        self.panes.push(Pane {
            pane_id: new_pane_id.clone(),
            active_tab_id: None,
        });

        new_pane_id
    }

    /// moves a tab from one pane to another
    fn move_tab(_tab_id: TabId, _pane_id: PaneId) {
        todo!()
    }

    fn get_tab_count_in_pane(&self, pane_id: &PaneId) -> usize {
        self.tabs
            .iter()
            .filter(|tab| tab.pane_id == *pane_id)
            .count()
    }

    fn get_tab_ids_and_names_of_pane(&self, pane_id: PaneId) -> Vec<(TabId, String)> {
        self.tabs
            .iter()
            .filter(|tab| tab.pane_id == pane_id)
            .map(|tab| (tab.tab_id, tab.tab_title.clone()))
            .collect()
    }

    /// Sets the tab as the foreground tab of a pane with the given tab_id
    fn set_active_tab(&mut self, tab_id: &TabId) {
        // the id of the pane for which the tab will be set as active
        let pane_id = self.get_pane_id_from_tab_id(tab_id);

        if let Some(pane) = self.panes.iter_mut().find(|pane| pane.pane_id == pane_id) {
            pane.active_tab_id = Some(*tab_id);
        }
    }
}

impl Default for Openstock {
    fn default() -> Self {
        let mut os = Self {
            panes: vec![],
            tabs: vec![],
            id_generator: IdGenerator::default(),
            context: Context {
                user: None,
                role: None,
                api: ApiClient::new("http://localhost:8080".to_string()).into(),
                theme: Theme::Dark,
            }
            .into(),
        };

        let pane_id = os.create_pane();
        os.create_tab_in_pane(CreateTab::Login, &pane_id);
        os
    }
}
