use anyhow::Result;
use iced::{
    keyboard::Modifiers,
    subscription,
    theme::Container,
    widget::{container, text, Column, Row},
    window, Application, Command, Element, Length, Settings, Subscription,
};
use style::{LineSelected, WordSelected};

fn main() -> Result<()> {
    Bismuth::run(Settings::default())?;
    Ok(())
}

struct Bismuth {
    code: Vec<Vec<String>>,
    sel: Selection,
}

struct Selection {
    line: usize,
    word: usize,
}

impl Bismuth {
    fn sel_line(&self) -> Option<&[String]> {
        self.code.get(self.sel.line).map(|v| v.as_slice())
    }

    fn sel_line_mut(&mut self) -> Option<&mut Vec<String>> {
        self.code.get_mut(self.sel.line)
    }

    fn sel_word(&self) -> Option<&str> {
        self.sel_line()
            .and_then(|line| line.get(self.sel.word).map(|word| word.as_str()))
    }

    fn sel_word_mut(&mut self) -> Option<&mut String> {
        let word = self.sel.word;
        self.sel_line_mut().and_then(|line| line.get_mut(word))
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
                code: vec![vec!["".into()]],
                sel: Selection { line: 0, word: 0 },
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
                CharacterReceived(letter @ 'a'..='z') => {
                    self.sel_word_mut().unwrap().push(letter);
                    Command::none()
                }
                KeyPressed {
                    key_code: Space,
                    modifiers: NONE,
                } => {
                    self.sel_line_mut().unwrap().push("".into());
                    self.sel.word += 1;
                    Command::none()
                }
                KeyPressed {
                    key_code: Backspace,
                    modifiers: NONE,
                } => {
                    self.sel_word_mut().unwrap().pop();
                    Command::none()
                }
                KeyPressed {
                    key_code: Enter,
                    modifiers: NONE,
                } => {
                    if !self.sel_word().unwrap().is_empty() {
                        self.code.insert(self.sel.line + 1, vec!["".into()]);
                        self.sel.line += 1;
                        self.sel.word = 0;
                    }
                    Command::none()
                }
                KeyPressed {
                    key_code: K,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.sel.line += 1;
                    self.sel.line = self.sel.line.min(self.code.len() - 1);
                    Command::none()
                }
                KeyPressed {
                    key_code: L,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.sel.line = self.sel.line.saturating_sub(1);
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
                .map(|(line_i, line)| (line_i == self.sel.line, line))
                .map(|(is_line_sel, line)| {
                    let line = Row::with_children(
                        line.iter()
                            .enumerate()
                            .map(|(word_i, word)| (is_line_sel && word_i == self.sel.word, word))
                            .map(|(is_word_sel, word)| {
                                let word = text(word).size(32).into();
                                if is_word_sel {
                                    container(word)
                                        .style(Container::Custom(Box::new(WordSelected)))
                                        .into()
                                } else {
                                    word
                                }
                            })
                            .collect(),
                    )
                    .spacing(8);
                    if is_line_sel {
                        container(line)
                            .style(Container::Custom(Box::new(LineSelected)))
                            .into()
                    } else {
                        line.into()
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

    pub struct WordSelected;

    impl container::StyleSheet for WordSelected {
        type Style = iced::Theme;

        fn appearance(&self, _style: &Self::Style) -> container::Appearance {
            container::Appearance {
                background: Some(Color::from_rgb8(43, 28, 61).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        }
    }
}
