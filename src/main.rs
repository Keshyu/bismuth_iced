use anyhow::Result;
use iced::{
    keyboard::Modifiers,
    subscription,
    widget::{container, text, Column},
    window, Application, Command, Element, Length, Settings, Subscription,
};

fn main() -> Result<()> {
    Bismuth::run(Settings::default())?;
    Ok(())
}

struct Bismuth {
    code: Vec<String>,
    selection: usize,
}

impl Application for Bismuth {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                code: vec!["".into()],
                selection: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Bismuth".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        use iced::event::Event::*;
        use iced::keyboard::Event::*;
        use iced::keyboard::KeyCode::*;
        use Message::*;

        const NONE: Modifiers = Modifiers::empty();

        match message {
            IcedEvent(Keyboard(k_event)) => match k_event {
                CharacterReceived(letter @ 'a'..='z') => {
                    self.code.last_mut().unwrap().push(letter);
                    Command::none()
                }
                KeyPressed {
                    key_code: Backspace,
                    modifiers: NONE,
                } => {
                    self.code.last_mut().unwrap().pop();
                    Command::none()
                }
                KeyPressed {
                    key_code: Enter,
                    modifiers: NONE,
                } => {
                    if !self.code.last().unwrap().is_empty() {
                        self.code.push("".into());
                    }
                    Command::none()
                }
                KeyPressed {
                    key_code: Q,
                    modifiers: Modifiers::SHIFT,
                } => window::close(),
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        container(Column::with_children(
            self.code
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    if i == self.selection {
                        container(text(line)).style(theme::C)
                    } else {
                        text(line).size(32)
                    }
                })
                .map(|(_, line)| line)
                .map(Element::from)
                .collect(),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::IcedEvent)
    }
}

#[derive(Debug)]
enum Message {
    IcedEvent(iced::Event),
}
