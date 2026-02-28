use crate::pages::Page;
use crate::Message;

#[derive(Clone)]
pub enum HomePageMessage {}

#[derive(Default)]
pub struct HomePage {}

impl Page for HomePage {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        todo!()
    }
}