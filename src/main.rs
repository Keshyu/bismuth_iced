use anyhow::Result;
use iced::{
    keyboard::Modifiers,
    subscription, theme,
    widget::{container, text, Column},
    window, Application, Command, Element, Length, Settings, Subscription,
};
use style::LineSelected;

fn main() -> Result<()> {
    Bismuth::run(Settings::default())?;
    Ok(())
}

struct Bismuth {
    code: Vec<String>,
    selection: usize,
}
impl Bismuth {
    fn selected_mut(&mut self) -> Option<&mut String> {
        self.code.get_mut(self.selection)
    }

    fn selected(&self) -> Option<&String> {
        self.code.get(self.selection)
    }
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
            window::maximize(true),
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
                CharacterReceived(letter @ ('a'..='z' | ' ')) => {
                    self.selected_mut().unwrap().push(letter);
                    Command::none()
                }
                KeyPressed {
                    key_code: Backspace,
                    modifiers: NONE,
                } => {
                    self.selected_mut().unwrap().pop();
                    Command::none()
                }
                KeyPressed {
                    key_code: Enter,
                    modifiers: NONE,
                } => {
                    if !self.selected().unwrap().is_empty() {
                        self.code.insert(self.selection + 1, "".into());
                        self.selection += 1;
                    }
                    Command::none()
                }
                KeyPressed {
                    key_code: K,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.selection += 1;
                    self.selection = self.selection.min(self.code.len() - 1);
                    Command::none()
                }
                KeyPressed {
                    key_code: L,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.selection = self.selection.saturating_sub(1);
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
                        container(text(line).size(32))
                            .style(theme::Container::Custom(Box::new(LineSelected)))
                            .into()
                    } else {
                        text(line).size(32).into()
                    }
                })
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

mod style {
    use iced::{widget::container, Color};

    pub struct LineSelected;

    impl container::StyleSheet for LineSelected {
        type Style = iced::Theme;

        fn appearance(&self, _style: &Self::Style) -> container::Appearance {
            container::Appearance {
                background: Some(Color::from_rgb8(71, 52, 94).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        }
    }
}
