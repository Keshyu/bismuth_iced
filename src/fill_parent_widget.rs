use iced::{
	advanced::{layout, renderer, Renderer as _, Widget},
	Color, Element, Length, Renderer, Size, Theme,
};

pub struct FillParent;

impl<Message> Widget<Message, Theme, Renderer> for FillParent {
	fn size(&self) -> iced::Size<iced::Length> {
		Size {
			width: Length::Fixed(8.),
			height: Length::Fill,
		}
	}

	fn layout(
		&self,
		_tree: &mut iced::advanced::widget::Tree,
		_renderer: &Renderer,
		limits: &iced::advanced::layout::Limits,
	) -> iced::advanced::layout::Node {
		layout::Node::new(Size::new(limits.max().width, limits.max().height))
	}

	fn draw(
		&self,
		_tree: &iced::advanced::widget::Tree,
		renderer: &mut Renderer,
		_theme: &Theme,
		_style: &iced::advanced::renderer::Style,
		layout: iced::advanced::Layout<'_>,
		_cursor: iced::advanced::mouse::Cursor,
		_viewport: &iced::Rectangle,
	) {
		renderer.fill_quad(
			renderer::Quad {
				bounds: layout.bounds(),
				..renderer::Quad::default()
			},
			Color::BLACK,
		);
	}
}

impl<'a, Message> From<FillParent> for Element<'a, Message> {
	fn from(value: FillParent) -> Self {
		Self::new(value)
	}
}
