use iced::theme::Theme;
use iced::widget::{container, text};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings};

fn main() -> iced::Result {
    Rixel::run(Settings::default())
}

struct Rixel {}

#[derive(Debug, Clone)]
enum Message {}

impl Application for Rixel {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Rixel {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rixel")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(text("Rixel"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
