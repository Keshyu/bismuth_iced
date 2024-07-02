//! Distribute content vertically.
use iced::advanced::layout;
use iced::advanced::layout::Limits;
use iced::advanced::layout::Node;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::event::{self, Event};
use iced::mouse;
use iced::Point;
use iced::{
	advanced::Clipboard, advanced::Layout, advanced::Shell, advanced::Widget, Element, Length,
	Padding, Pixels, Rectangle, Size, Vector,
};

/// Alignment on the axis of a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
	/// Align at the start of the axis.
	Start,

	/// Align at the center of the axis.
	Center,

	/// Align at the end of the axis.
	End,

	Stretch,
}

/// The main axis of a flex layout.
#[derive(Debug)]
pub enum Axis {
	/// The horizontal axis
	Horizontal,

	/// The vertical axis
	Vertical,
}

impl Axis {
	fn main(&self, size: Size) -> f32 {
		match self {
			Axis::Horizontal => size.width,
			Axis::Vertical => size.height,
		}
	}

	fn cross(&self, size: Size) -> f32 {
		match self {
			Axis::Horizontal => size.height,
			Axis::Vertical => size.width,
		}
	}

	fn pack<T>(&self, main: T, cross: T) -> (T, T) {
		match self {
			Axis::Horizontal => (main, cross),
			Axis::Vertical => (cross, main),
		}
	}
}

/// A container that distributes its contents vertically.
#[allow(missing_debug_implementations)]
pub struct Column<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
	spacing: f32,
	padding: Padding,
	width: Length,
	height: Length,
	max_width: f32,
	align_items: Alignment,
	clip: bool,
	children: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Column<'a, Message, Theme, Renderer>
where
	Renderer: iced::advanced::Renderer,
{
	/// Creates an empty [`Column`].
	pub fn new() -> Self {
		Self::from_vec(Vec::new())
	}

	/// Creates a [`Column`] with the given elements.
	pub fn with_children(
		children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
	) -> Self {
		Self::new().extend(children)
	}

	/// Creates a [`Column`] from an already allocated [`Vec`].
	///
	/// Keep in mind that the [`Column`] will not inspect the [`Vec`], which means
	/// it won't automatically adapt to the sizing strategy of its contents.
	///
	/// If any of the children have a [`Length::Fill`] strategy, you will need to
	/// call [`Column::width`] or [`Column::height`] accordingly.
	pub fn from_vec(children: Vec<Element<'a, Message, Theme, Renderer>>) -> Self {
		Self {
			spacing: 0.0,
			padding: Padding::ZERO,
			width: Length::Shrink,
			height: Length::Shrink,
			max_width: f32::INFINITY,
			align_items: Alignment::Start,
			clip: false,
			children,
		}
	}

	/// Sets the vertical spacing _between_ elements.
	///
	/// Custom margins per element do not exist in iced. You should use this
	/// method instead! While less flexible, it helps you keep spacing between
	/// elements consistent.
	pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
		self.spacing = amount.into().0;
		self
	}

	/// Sets the [`Padding`] of the [`Column`].
	pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
		self.padding = padding.into();
		self
	}

	/// Sets the width of the [`Column`].
	pub fn width(mut self, width: impl Into<Length>) -> Self {
		self.width = width.into();
		self
	}

	/// Sets the height of the [`Column`].
	pub fn height(mut self, height: impl Into<Length>) -> Self {
		self.height = height.into();
		self
	}

	/// Sets the maximum width of the [`Column`].
	pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
		self.max_width = max_width.into().0;
		self
	}

	/// Sets the horizontal alignment of the contents of the [`Column`] .
	pub fn align_items(mut self, align: Alignment) -> Self {
		self.align_items = align;
		self
	}

	/// Sets whether the contents of the [`Column`] should be clipped on
	/// overflow.
	pub fn clip(mut self, clip: bool) -> Self {
		self.clip = clip;
		self
	}

	/// Adds an element to the [`Column`].
	pub fn push(mut self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
		let child = child.into();
		let size = child.as_widget().size_hint();

		if size.width.is_fill() {
			self.width = Length::Fill;
		}

		if size.height.is_fill() {
			self.height = Length::Fill;
		}

		self.children.push(child);
		self
	}

	/// Adds an element to the [`Column`], if `Some`.
	pub fn push_maybe(
		self,
		child: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
	) -> Self {
		if let Some(child) = child {
			self.push(child)
		} else {
			self
		}
	}

	/// Extends the [`Column`] with the given children.
	pub fn extend(
		self,
		children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
	) -> Self {
		children.into_iter().fold(self, Self::push)
	}
}

impl<'a, Message, Renderer> Default for Column<'a, Message, Renderer>
where
	Renderer: iced::advanced::Renderer,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
	for Column<'a, Message, Theme, Renderer>
where
	Renderer: iced::advanced::Renderer,
{
	fn children(&self) -> Vec<Tree> {
		self.children.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree) {
		tree.diff_children(&self.children);
	}

	fn size(&self) -> Size<Length> {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn layout(
		&self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node {
		let limits = limits
			.max_width(self.max_width)
			.width(self.width)
			.height(self.height)
			.shrink(self.padding);
		let axis = Axis::Vertical;
		let trees = &mut tree.children;
		let total_spacing = self.spacing * self.children.len().saturating_sub(1) as f32;
		let max_cross = axis.cross(limits.max());

		let mut fill_main_sum = 0;
		let mut cross = match axis {
			Axis::Vertical if self.width == Length::Shrink => 0.0,
			Axis::Horizontal if self.height == Length::Shrink => 0.0,
			_ => max_cross,
		};

		let mut available = axis.main(limits.max()) - total_spacing;

		let mut nodes = vec![Node::default(); self.children.len()];

		for (i, (child, tree)) in self.children.iter().zip(trees.iter_mut()).enumerate() {
			let (fill_main_factor, fill_cross_factor) = {
				let size = child.as_widget().size();
				axis.pack(size.width.fill_factor(), size.height.fill_factor())
			};

			if fill_main_factor == 0 {
				let (max_width, max_height) = axis.pack(
					available,
					if fill_cross_factor == 0 {
						max_cross
					} else {
						cross
					},
				);

				let layout = child.as_widget().layout(
					tree,
					renderer,
					&Limits::new(Size::ZERO, Size::new(max_width, max_height)),
				);

				available -= axis.main(layout.size());
				cross = cross.max(axis.cross(layout.size()));
				nodes[i] = layout;
			} else {
				fill_main_sum += fill_main_factor;
			}
		}

		let remaining = match axis {
			Axis::Horizontal => match self.width {
				Length::Shrink => 0.0,
				_ => available.max(0.0),
			},
			Axis::Vertical => match self.height {
				Length::Shrink => 0.0,
				_ => available.max(0.0),
			},
		};

		for (i, (child, tree)) in self.children.iter().zip(trees).enumerate() {
			let (fill_main_factor, fill_cross_factor) = {
				let size = child.as_widget().size();
				axis.pack(size.width.fill_factor(), size.height.fill_factor())
			};

			if fill_main_factor != 0 {
				let max_main = remaining * fill_main_factor as f32 / fill_main_sum as f32;

				let (max_width, max_height) = axis.pack(
					max_main,
					if fill_cross_factor == 0 {
						max_cross
					} else {
						cross
					},
				);
				let (min_width, min_height) = axis.pack(max_main, 0.0);

				let layout = child.as_widget().layout(
					tree,
					renderer,
					&Limits::new(
						Size::new(min_width, min_height),
						Size::new(max_width, max_height),
					),
				);

				cross = cross.max(axis.cross(layout.size()));
				nodes[i] = layout;
			}
		}

		let pad = axis.pack(self.padding.left, self.padding.top);
		let mut main = pad.0;

		for (i, node) in nodes.iter_mut().enumerate() {
			if i > 0 {
				main += self.spacing;
			}

			let (mut x, mut y) = axis.pack(main, pad.1);
			let (horizontal_alignment, vertical_alignment) =
				axis.pack(Alignment::Start, self.align_items);
			let space = Size::new(0.0, cross);

			match horizontal_alignment {
				Alignment::Start => {}
				Alignment::Center => {
					x += (space.width - node.size().width) / 2.0;
				}
				Alignment::End => {
					x += space.width - node.size().width;
				}
				Alignment::Stretch => todo!(),
			}

			match vertical_alignment {
				Alignment::Start => {}
				Alignment::Center => {
					y += (space.height - node.size().height) / 2.0;
				}
				Alignment::End => {
					y += space.height - node.size().height;
				}
				Alignment::Stretch => todo!(),
			};

			node.move_to_mut(Point::new(x, y));
			main += axis.main(node.size());
		}

		let (intrinsic_width, intrinsic_height) = axis.pack(main - pad.0, cross);
		let size = limits.resolve(
			self.width,
			self.height,
			Size::new(intrinsic_width, intrinsic_height),
		);

		Node::with_children(size.expand(self.padding), nodes)
	}

	fn operate(
		&self,
		tree: &mut Tree,
		layout: Layout<'_>,
		renderer: &Renderer,
		operation: &mut dyn Operation<Message>,
	) {
		operation.container(None, layout.bounds(), &mut |operation| {
			self.children
				.iter()
				.zip(&mut tree.children)
				.zip(layout.children())
				.for_each(|((child, state), layout)| {
					child
						.as_widget()
						.operate(state, layout, renderer, operation);
				});
		});
	}

	fn on_event(
		&mut self,
		tree: &mut Tree,
		event: Event,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	) -> event::Status {
		self.children
			.iter_mut()
			.zip(&mut tree.children)
			.zip(layout.children())
			.map(|((child, state), layout)| {
				child.as_widget_mut().on_event(
					state,
					event.clone(),
					layout,
					cursor,
					renderer,
					clipboard,
					shell,
					viewport,
				)
			})
			.fold(event::Status::Ignored, event::Status::merge)
	}

	fn mouse_interaction(
		&self,
		tree: &Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
		renderer: &Renderer,
	) -> mouse::Interaction {
		self.children
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
			.map(|((child, state), layout)| {
				child
					.as_widget()
					.mouse_interaction(state, layout, cursor, viewport, renderer)
			})
			.max()
			.unwrap_or_default()
	}

	fn draw(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
	) {
		if let Some(clipped_viewport) = layout.bounds().intersection(viewport) {
			for ((child, state), layout) in self
				.children
				.iter()
				.zip(&tree.children)
				.zip(layout.children())
			{
				child.as_widget().draw(
					state,
					renderer,
					theme,
					style,
					layout,
					cursor,
					if self.clip {
						&clipped_viewport
					} else {
						viewport
					},
				);
			}
		}
	}

	fn overlay<'b>(
		&'b mut self,
		tree: &'b mut Tree,
		layout: Layout<'_>,
		renderer: &Renderer,
		translation: Vector,
	) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
		overlay::from_children(&mut self.children, tree, layout, renderer, translation)
	}
}

impl<'a, Message, Theme, Renderer> From<Column<'a, Message, Theme, Renderer>>
	for Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: 'a,
	Renderer: iced::advanced::Renderer + 'a,
{
	fn from(column: Column<'a, Message, Theme, Renderer>) -> Self {
		Self::new(column)
	}
}
