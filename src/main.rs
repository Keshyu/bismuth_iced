mod code;
mod code_widget;
mod ui;

use anim::{easing::EasingMode, Options, Timeline};
use iced::{
	color,
	widget::{column, container, row, svg, text, Space, Svg},
	window, Application, ContentFit, Element, Length, Settings, Subscription,
};
use std::time::Duration;

fn main() -> Result<(), iced::Error> {
	CodeEditor::run(Settings::default())
}

struct CodeEditor {
	code: code::Code,
	sel: code::NodeId,
	timeline: Timeline<f32>,
}

impl Application for CodeEditor {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = iced::Theme;
	type Flags = ();

	fn new((): Self::Flags) -> (Self, iced::Command<Message>) {
		let mut code = code::Code::new();
		{
			use code::Node;

			code.insert(Node::Word("main".into()), code.root()).unwrap();
			let main = code.insert(Node::Group(Vec::new()), code.root()).unwrap();
			code.insert(Node::Word("allocator".into()), main).unwrap();
			code.insert(Node::Word("setup".into()), main).unwrap();
			code.insert(Node::Word("game-loop".into()), main).unwrap();

			let game_loop = code.insert(Node::Group(Vec::new()), main).unwrap();
			code.insert(Node::Word("clear-bg".into()), game_loop)
				.unwrap();
			code.insert(Node::Word("dialogue-timer".into()), game_loop)
				.unwrap();
			code.insert(Node::Word("dialogue-display".into()), game_loop)
				.unwrap();

			code.insert(Node::Word("window".into()), code.root())
				.unwrap();
			let window = code.insert(Node::Group(Vec::new()), code.root()).unwrap();
			code.insert(Node::Word("width".into()), window).unwrap();
			code.insert(Node::Word("height".into()), window).unwrap();
			code.insert(Node::Word("title".into()), window).unwrap();

			code.insert(Node::Word("import".into()), code.root())
				.unwrap();
			let import = code.insert(Node::Group(Vec::new()), code.root()).unwrap();
			code.insert(Node::Word("mem".into()), import).unwrap();
			code.insert(Node::Word("strings".into()), import).unwrap();
			code.insert(Node::Word("raylib".into()), import).unwrap();
		}
		(
			Self {
				sel: code.root(),
				code,
				timeline: Options::new(1.0, 0.1)
					.duration(Duration::from_millis(2000))
					.easing(anim::easing::cubic_ease().mode(EasingMode::InOut))
					.auto_reverse(true)
					.delay(Duration::from_millis(400))
					.forever()
					.begin_animation(),
			},
			window::maximize(window::Id::MAIN, true),
		)
	}

	fn title(&self) -> String {
		"Bismuth Editor".into()
	}

	fn view(&self) -> Element<Message> {
		fn view_node(node: code::NodeId, code: &code::Code) -> Element<Message> {
			use code::Node;
			match code.get(node).unwrap() {
				Node::Word(word) => text(word).size(24).into(),
				Node::Group(group) => row![
					column![
						Space::with_height(8),
						Svg::from_path("assets/branch_top.svg").width(5).style(
							iced::theme::Svg::custom_fn(|_| svg::Appearance {
								color: Some(color!(0x8B6DB0).into()),
							})
						),
						Svg::from_path("assets/branch_mid.svg")
							.height(Length::Fill)
							.width(1)
							.content_fit(ContentFit::Fill)
							.style(iced::theme::Svg::custom_fn(|_| svg::Appearance {
								color: Some(color!(0x8B6DB0).into()),
							})),
						Svg::from_path("assets/branch_bottom.svg").width(5).style(
							iced::theme::Svg::custom_fn(|_| svg::Appearance {
								color: Some(color!(0x8B6DB0).into()),
							})
						),
					]
					.padding([8, 0]),
					Space::with_width(32),
					column(group.iter().map(|node_id| view_node(*node_id, code))).spacing(6),
				]
				.into(),
			}
		}

		container(view_node(self.code.root(), &self.code))
			// container(
			// 	Column::with_children(
			// 		self.code
			// 			.iter()
			// 			.enumerate()
			// 			.map(|(line_i, line)| (line_i == self.sel.line, line))
			// 			.map(|(is_line_sel, line)| {
			// 				let line = Row::with_children(
			// 					line.iter()
			// 						.enumerate()
			// 						.map(|(word_i, word)| {
			// 							(is_line_sel && word_i == self.sel.word, word)
			// 						})
			// 						.map(|(is_word_sel, word)| {
			// 							let word = container(text(word).size(28));
			// 							if is_word_sel {
			// 								container(
			// 									container(word)
			// 										.padding([4 + 0, 6 + 4])
			// 										.style(style::word_selected(self.timeline.value())),
			// 								)
			// 								.padding([-4.0, -6.0])
			// 								.into()
			// 							} else {
			// 								container(word).padding([0, 4]).into()
			// 							}
			// 						})
			// 						.collect::<Vec<_>>(),
			// 				)
			// 				.spacing(8);
			// 				if is_line_sel {
			// 					container(
			// 						container(line)
			// 							.padding([4 + 0, 6])
			// 							.style(style::line_selected),
			// 					)
			// 					.padding([-4.0, -6.0])
			// 					.into()
			// 				} else {
			// 					container(line).padding([0, 0]).into()
			// 				}
			// 			})
			// 			.collect::<Vec<_>>(),
			// 	)
			// 	.spacing(10),
			// )
			.width(Length::Fill)
			.height(Length::Fill)
			.center_y()
			.padding([12, 36, 0, 36])
			.style(style::screen)
			.into()
	}

	fn update(&mut self, message: Message) -> iced::Command<Message> {
		use iced::keyboard::key::Named;
		use iced::keyboard::Event::*;
		use iced::keyboard::Key;
		use iced::keyboard::Modifiers;

		const NONE: Modifiers = Modifiers::empty();

		match message {
			// Message::Keyboard(e) => match e {
			// 	KeyPressed {
			// 		key: Key::Named(Named::Backspace),
			// 		modifiers: NONE,
			// 		..
			// 	} => {
			// 		self.code
			// 			.get_mut(self.sel.line)
			// 			.unwrap()
			// 			.get_mut(self.sel.word)
			// 			.unwrap()
			// 			.pop();
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "k" => {
			// 		self.set_sel(Selection {
			// 			line: self.sel.line + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "l" => {
			// 		self.set_sel(Selection {
			// 			line: self.sel.line.saturating_sub(1),
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "j" => {
			// 		self.set_sel(Selection {
			// 			word: self.sel.word.saturating_sub(1),
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == ";" => {
			// 		self.set_sel(Selection {
			// 			word: self.sel.word + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "i" => {
			// 		self.code.insert(self.sel.line + 1, vec!["".into()]);
			// 		self.set_sel(Selection {
			// 			line: self.sel.line + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "o" => {
			// 		self.code.insert(self.sel.line, vec!["".into()]);
			// 		self.sel.word = 0;
			// 		self.remove_if_empty(&Selection {
			// 			line: self.sel.line + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Named(Named::Space),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} => {
			// 		let word = self.sel.word;
			// 		self.code
			// 			.get_mut(self.sel.line)
			// 			.unwrap()
			// 			.insert(word, "".into());
			// 		self.remove_if_empty(&Selection {
			// 			word: self.sel.word + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Named(Named::Space),
			// 		modifiers: NONE,
			// 		..
			// 	} => {
			// 		let word = self.sel.word;
			// 		self.code
			// 			.get_mut(self.sel.line)
			// 			.unwrap()
			// 			.insert(word + 1, "".into());
			// 		self.set_sel(Selection {
			// 			word: self.sel.word + 1,
			// 			..self.sel
			// 		});
			// 		iced::Command::none()
			// 	}
			// 	KeyPressed {
			// 		key: Key::Character(c),
			// 		modifiers: Modifiers::SHIFT,
			// 		..
			// 	} if c.as_str() == "q" => window::close(window::Id::MAIN),
			// 	KeyPressed {
			// 		text: Some(text),
			// 		modifiers: NONE,
			// 		..
			// 	} if text
			// 		.chars()
			// 		.nth(0)
			// 		.is_some_and(|c| c.is_ascii_lowercase() || c == '.') =>
			// 	{
			// 		self.code
			// 			.get_mut(self.sel.line)
			// 			.unwrap()
			// 			.get_mut(self.sel.word)
			// 			.unwrap()
			// 			.push(text.chars().nth(0).unwrap());
			// 		iced::Command::none()
			// 	}
			// 	_ => iced::Command::none(),
			// },
			Message::Keyboard(..) => iced::Command::none(),
			Message::Tick => {
				self.timeline.update();
				iced::Command::none()
			}
		}
	}

	fn subscription(&self) -> Subscription<Message> {
		const FPS: f32 = 60.0;
		Subscription::batch([
			iced::event::listen_with(|event, _| match event {
				iced::Event::Keyboard(e) => Some(Message::Keyboard(e)),
				_ => None,
			}),
			iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_| Message::Tick),
		])
	}
}

// impl CodeEditor {
// 	fn set_sel(&mut self, sel: Selection) {
// 		let mut old_sel = sel;
// 		std::mem::swap(&mut old_sel, &mut self.sel);

// 		self.sel.line = self.sel.line.clamp(0, self.code.len() - 1);

// 		if old_sel.line != self.sel.line {
// 			self.sel.word = 0;
// 		} else {
// 			self.sel.word = self
// 				.sel
// 				.word
// 				.clamp(0, self.code.get(self.sel.line).unwrap().len() - 1);
// 		}

// 		if old_sel != self.sel {
// 			self.remove_if_empty(&old_sel);
// 		}
// 	}

// 	fn remove(&mut self, pos: &Selection) {
// 		// ASSUME: `pos` is in-bounds

// 		self.code.get_mut(pos.line).unwrap().remove(pos.word);
// 		if self.sel.line == pos.line && self.sel.word >= pos.word {
// 			self.sel.word = self.sel.word.saturating_sub(1);
// 		}
// 		if self.code.get(pos.line).unwrap().is_empty() {
// 			self.code.remove(pos.line);
// 			if self.sel.line >= pos.line {
// 				self.sel.line = self.sel.line.saturating_sub(1);
// 			}
// 		}
// 		if self.code.is_empty() {
// 			self.code.push(vec!["".into()]);
// 		}
// 	}

// 	fn remove_if_empty(&mut self, pos: &Selection) {
// 		// ASSUME: `pos` is in-bounds

// 		if self.get(pos).unwrap().is_empty() {
// 			self.remove(pos);
// 		}
// 	}

// 	fn get(&self, pos: &Selection) -> Option<&str> {
// 		Some(self.code.get(pos.line)?.get(pos.word)?.as_str())
// 	}
// }

#[derive(Debug)]
enum Message {
	Keyboard(iced::keyboard::Event),
	Tick,
}

mod style {
	use iced::border::Radius;
	use iced::{color, Border};
	use iced::{widget::container, Color, Theme};

	pub fn screen(_: &Theme) -> container::Appearance {
		container::Appearance {
			background: Some(color!(0x322442).into()),
			text_color: Some(Color::WHITE),
			..Default::default()
		}
	}

	pub fn line_selected(_: &Theme) -> container::Appearance {
		container::Appearance {
			background: Some(color!(0x281D34).into()),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: Radius::from(6),
			},
			..Default::default()
		}
	}

	pub fn word_selected(opacity: f32) -> impl Fn(&Theme) -> container::Appearance {
		move |_| container::Appearance {
			background: Some(color!(0x6E5091, opacity).into()),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: Radius::from(6),
			},
			..Default::default()
		}
	}
}
