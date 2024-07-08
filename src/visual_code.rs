use crate::code::Code;

pub struct VisualCode {
	code: Code,
}

impl VisualCode {
	fn update(&mut self, code: &mut Code, message: Message) {
		todo!()
	}

	fn view(&self, code: &Code) -> iced::Element<Message> {
		todo!()
	}
}

pub enum Message {}
