pub mod home;
pub mod login;
pub mod window;

//use crate::Context;
//use crate::Message;
//use iced::Task;

//pub trait Window {
//    fn update(
//        &mut self,
//        message: Message,
//        app: &Context,
//    ) -> (Option<Box<dyn Window>>, Task<Message>);
//    fn view(&self) -> iced::Element<'_, Message>;
//}

pub use login::LoginWindow;
