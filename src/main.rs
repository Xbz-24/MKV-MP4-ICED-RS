use iced::{
    Alignment, Element, Sandbox, Settings,
};

use iced::widget::{
    button, column, container, row
};

use iced::theme::Theme;

pub fn main() -> iced::Result {
    Styling::run(Settings::default())
}

#[derive(Default)]
struct Styling {
    theme: Theme,
    is_dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling {
            theme: Theme::Dark,
            is_dark: true,
        }
    }

    fn title(&self) -> String {
        String::from("Iced mp4->mp3 converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.is_dark = !self.is_dark;
                self.theme = if self.is_dark {
                    Theme::Dark
                } else {
                    Theme::Light
                };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let button = button::Button::new("Toggle Theme")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let content = column![row![button].align_items(Alignment::Center)];

        container(content).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
