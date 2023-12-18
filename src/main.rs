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

#[derive(PartialEq)]
struct Selection {
    line: usize,
    word: usize,
}

impl Bismuth {
    fn set_sel(&mut self, sel: Selection) {
        let mut old_sel = sel;
        std::mem::swap(&mut old_sel, &mut self.sel);

        self.sel.line = self.sel.line.clamp(0, self.code.len() - 1);

        if old_sel.line != self.sel.line {
            self.sel.word = 0;
        } else {
            self.sel.word = self
                .sel
                .word
                .clamp(0, self.code.get(self.sel.line).unwrap().len() - 1);
        }

        if old_sel != self.sel {
            self.remove_if_empty(&old_sel);
        }
    }

    fn remove(&mut self, pos: &Selection) {
        // ASSUME: pos is in-bounds

        self.code.get_mut(pos.line).unwrap().remove(pos.word);
        if self.sel.line == pos.line && self.sel.word >= pos.word {
            self.sel.word = self.sel.word.saturating_sub(1);
        }
        if self.code.get(pos.line).unwrap().len() == 0 {
            self.code.remove(pos.line);
            if self.sel.line >= pos.line {
                self.sel.line = self.sel.line.saturating_sub(1);
            }
        }
        if self.code.is_empty() {
            self.code.push(vec!["".into()]);
        }
    }

    fn remove_if_empty(&mut self, pos: &Selection) {
        if self.get(pos).unwrap().is_empty() {
            self.remove(pos);
        }
    }

    fn get(&self, pos: &Selection) -> Option<&str> {
        Some(self.code.get(pos.line)?.get(pos.word)?.as_str())
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
                    self.code
                        .get_mut(self.sel.line)
                        .unwrap()
                        .get_mut(self.sel.word)
                        .unwrap()
                        .push(letter);
                    Command::none()
                }
                KeyPressed {
                    key_code: Backspace,
                    modifiers: NONE,
                } => {
                    self.code
                        .get_mut(self.sel.line)
                        .unwrap()
                        .get_mut(self.sel.word)
                        .unwrap()
                        .pop();
                    Command::none()
                }
                KeyPressed {
                    key_code: K,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.set_sel(Selection {
                        line: self.sel.line + 1,
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: L,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.set_sel(Selection {
                        line: self.sel.line.saturating_sub(1),
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: J,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.set_sel(Selection {
                        word: self.sel.word.saturating_sub(1),
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: Semicolon,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.set_sel(Selection {
                        word: self.sel.word + 1,
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: I,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.code.insert(self.sel.line + 1, vec!["".into()]);
                    self.set_sel(Selection {
                        line: self.sel.line + 1,
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: O,
                    modifiers: Modifiers::SHIFT,
                } => {
                    self.code.insert(self.sel.line, vec!["".into()]);
                    self.remove_if_empty(&Selection {
                        line: self.sel.line + 1,
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: Space,
                    modifiers: Modifiers::SHIFT,
                } => {
                    let word = self.sel.word;
                    self.code
                        .get_mut(self.sel.line)
                        .unwrap()
                        .insert(word, "".into());
                    self.remove_if_empty(&Selection {
                        word: self.sel.word + 1,
                        ..self.sel
                    });
                    Command::none()
                }
                KeyPressed {
                    key_code: Space,
                    modifiers: NONE,
                } => {
                    let word = self.sel.word;
                    self.code
                        .get_mut(self.sel.line)
                        .unwrap()
                        .insert(word + 1, "".into());
                    self.set_sel(Selection {
                        word: self.sel.word + 1,
                        ..self.sel
                    });
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
