// mod code;
// mod visual_code;
mod code_widget;
mod fill_parent_widget;
mod ui;

use anim::{easing::EasingMode, Options, Timeline};
use fill_parent_widget::FillParent;
use iced::{
	color,
	widget::{
		column, container, responsive, row, scrollable, svg, text, vertical_rule, Rule, Space, Svg,
	},
	window, Application, ContentFit, Element, Length, Settings, Subscription,
};
use std::time::Duration;

fn main() -> Result<(), iced::Error> {
	CodeEditor::run(Settings::default())
}

struct CodeEditor {
	// code: code::Code,
	timeline: Timeline<f32>,
}

impl Application for CodeEditor {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = iced::Theme;
	type Flags = ();

	fn new((): Self::Flags) -> (Self, iced::Command<Message>) {
		(
			Self {
				// code,
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
		container(scrollable(
			container(row![
				FillParent,
				column![
					text("Hello"),
					text("Hello"),
					text("Hello"),
					text("Hello"),
					text("Hello"),
				],
			])
			.width(Length::Fill)
			.padding([24, 36]),
		))
		.height(Length::Fill)
		.center_y()
		.style(style::screen)
		.into()
	}

	fn update(&mut self, message: Message) -> iced::Command<Message> {
		iced::Command::none()
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
