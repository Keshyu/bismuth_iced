use apply::Apply;
use iced::Color;
use std::collections::HashMap;

pub fn resolve(constraints: Vec<Constraint>) -> Result<Vec<Element>, ResolutionError> {
	let element_constraints = constraints
		.apply(extract_prop_constraints)
		.apply(group_by_element)
		.apply(fill_with_inherent_constraints);

	todo!()
}

fn extract_prop_constraints(
	constraints: impl IntoIterator<Item = Constraint>,
) -> impl Iterator<Item = Constraint> {
	constraints.into_iter().flat_map(|constraint| {
		find_paths_to_vars(&constraint)
			.into_iter()
			.map(|path| isolate_variable(path, constraint.clone()))
			.collect::<Vec<Constraint>>()
	})
}

fn find_paths_to_vars(constraint: &Constraint) -> Vec<Vec<Direction>> {
	use Direction as D;
	use Expression as E;

	let mut paths = Vec::<Vec<Direction>>::new();
	let mut current_path = Vec::<Option<Direction>>::new();
	let mut trail = Vec::<&Expression>::new();

	let Constraint::Equality { left, right } = constraint;

	for (expr, dir) in [(left, D::Left), (right, D::Right)] {
		current_path.push(Some(dir));
		current_path.push(None);
		trail.push(expr);

		loop {
			match (trail.last(), current_path.last().unwrap()) {
				(Some(E::Sum(left, _) | E::Product(left, _)), None) => {
					*current_path.last_mut().unwrap() = Some(D::Left);
					current_path.push(None);
					trail.push(left);
				}
				(Some(E::Sum(_, right) | E::Product(_, right)), Some(D::Left)) => {
					*current_path.last_mut().unwrap() = Some(D::Right);
					current_path.push(None);
					trail.push(right);
				}
				(Some(E::Sum(_, _) | E::Product(_, _)), Some(D::Right)) => {
					current_path.pop();
					trail.pop();
				}
				(Some(E::Negative(expression) | E::Fraction(expression)), None) => {
					*current_path.last_mut().unwrap() = Some(D::Down);
					current_path.push(None);
					trail.push(expression);
				}
				(Some(E::Value(_)), None) => {
					current_path.pop();
					trail.pop();
				}
				(Some(E::Property(_, _)), None) => {
					current_path.pop();
					paths.push(
						current_path
							.clone()
							.into_iter()
							.collect::<Option<Vec<_>>>()
							.unwrap(),
					);
					trail.pop();
				}
				(None, _) => {
					break;
				}
				(Some(E::Sum(_, _) | E::Product(_, _)), _)
				| (Some(E::Negative(_) | E::Fraction(_)), _)
				| (Some(E::Value(_)), _)
				| (Some(E::Property(_, _)), _) => panic!(),
			}
		}

		current_path.pop();
	}

	paths
}

fn isolate_variable(path: Vec<Direction>, init: Constraint) -> Constraint {
	use Expression as Expr;

	let mut path = path.iter();
	let Constraint::Equality { left, right } = init;

	let mut extract_from: Option<Expr>;
	let mut throw_into: Option<Expr>;

	match path.next() {
		Some(Direction::Left) => {
			extract_from = Some(left);
			throw_into = Some(right);
		}
		Some(Direction::Right) => {
			extract_from = Some(right);
			throw_into = Some(left);
		}
		_ => panic!(),
	}

	for direction in path {
		match (extract_from.unwrap(), direction) {
			(Expr::Sum(left, right), Direction::Left) => {
				throw_into = Some(Expr::Sum(
					Box::new(throw_into.take().unwrap()),
					Box::new(Expr::Negative(right)),
				));
				extract_from = Some(*left);
			}
			(Expr::Sum(left, right), Direction::Right) => {
				throw_into = Some(Expr::Sum(
					Box::new(throw_into.take().unwrap()),
					Box::new(Expr::Negative(left)),
				));
				extract_from = Some(*right);
			}
			(Expr::Product(left, right), Direction::Left) => {
				throw_into = Some(Expr::Product(
					Box::new(throw_into.take().unwrap()),
					Box::new(Expr::Fraction(right)),
				));
				extract_from = Some(*left);
			}
			(Expr::Product(left, right), Direction::Right) => {
				throw_into = Some(Expr::Product(
					Box::new(throw_into.take().unwrap()),
					Box::new(Expr::Fraction(left)),
				));
				extract_from = Some(*right);
			}
			(Expr::Negative(inner), Direction::Down) => {
				throw_into = Some(Expr::Negative(Box::new(throw_into.take().unwrap())));
				extract_from = Some(*inner);
			}
			(Expr::Fraction(inner), Direction::Down) => {
				throw_into = Some(Expr::Fraction(Box::new(throw_into.take().unwrap())));
				extract_from = Some(*inner);
			}
			(Expr::Sum(_, _), _)
			| (Expr::Product(_, _), _)
			| (Expr::Negative(_), _)
			| (Expr::Fraction(_), _)
			| (Expr::Value(_), _)
			| (Expr::Property(_, _), _) => panic!(),
		}
	}

	Constraint::Equality {
		left: extract_from.unwrap(),
		right: throw_into.unwrap(),
	}
}

fn group_by_element(
	constraints: impl IntoIterator<Item = Constraint>,
) -> HashMap<Name, ElementConstraints> {
	let mut elements = HashMap::<Name, ElementConstraints>::new();

	constraints
		.into_iter()
		.for_each(|constraint| match constraint {
			Constraint::Equality {
				left: Expression::Property(element_name, property),
				right: expression,
			} => {
				let element = elements.entry(element_name).or_default();
				element.get_property(property).push(expression);
			}
			_ => panic!(),
		});

	elements
}

fn fill_with_inherent_constraints(
	mut elements: HashMap<Name, ElementConstraints>,
) -> HashMap<Name, ElementConstraints> {
	elements.iter_mut().for_each(|(name, mut element)| {
		use Expression as E;

		element.width.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Right)),
			Box::new(E::Negative(Box::new(E::Property(
				name.clone(),
				Property::Left,
			)))),
		));

		element.left.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Right)),
			Box::new(E::Negative(Box::new(E::Property(
				name.clone(),
				Property::Width,
			)))),
		));

		element.right.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Left)),
			Box::new(E::Property(name.clone(), Property::Width)),
		));

		element.height.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Top)),
			Box::new(E::Negative(Box::new(E::Property(
				name.clone(),
				Property::Bottom,
			)))),
		));

		element.bottom.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Top)),
			Box::new(E::Negative(Box::new(E::Property(
				name.clone(),
				Property::Height,
			)))),
		));

		element.top.push(E::Sum(
			Box::new(E::Property(name.clone(), Property::Bottom)),
			Box::new(E::Property(name.clone(), Property::Height)),
		));

		// TODO: x, y, center
	});
	elements
}

#[derive(Clone)]
pub enum Constraint {
	Equality { left: Expression, right: Expression },
}

#[derive(Clone)]
pub enum Expression {
	Sum(Box<Expression>, Box<Expression>),
	Product(Box<Expression>, Box<Expression>),
	Negative(Box<Expression>),
	Fraction(Box<Expression>),
	Value(i32),
	Property(Name, Property),
}

pub struct Element {
	pub width: i32,
	pub height: i32,
	pub left: i32,
	pub right: i32,
	pub bottom: i32,
	pub top: i32,
	pub x: i32,
	pub y: i32,
	pub center: (i32, i32),
	pub color: Color,
}

#[derive(Default)]
pub struct ElementBuilder {
	pub width: Option<i32>,
	pub height: Option<i32>,
	pub left: Option<i32>,
	pub right: Option<i32>,
	pub bottom: Option<i32>,
	pub top: Option<i32>,
	pub x: Option<i32>,
	pub y: Option<i32>,
	pub center: Option<(i32, i32)>,
	pub color: Option<Color>,
}

#[derive(Default)]
pub struct ElementConstraints {
	pub width: Vec<Expression>,
	pub height: Vec<Expression>,
	pub left: Vec<Expression>,
	pub right: Vec<Expression>,
	pub bottom: Vec<Expression>,
	pub top: Vec<Expression>,
	pub x: Vec<Expression>,
	pub y: Vec<Expression>,
	pub center: Vec<Expression>,
	pub color: Vec<Expression>,
}

impl ElementConstraints {
	pub fn get_property(&mut self, prop: Property) -> &mut Vec<Expression> {
		match prop {
			Property::Width => &mut self.width,
			Property::Height => &mut self.height,
			Property::Left => &mut self.left,
			Property::Right => &mut self.right,
			Property::Bottom => &mut self.bottom,
			Property::Top => &mut self.top,
			Property::X => &mut self.x,
			Property::Y => &mut self.y,
			Property::Center => &mut self.center,
			Property::Color => &mut self.color,
		}
	}
}

#[derive(Clone)]
pub enum Property {
	Width,
	Height,
	Left,
	Right,
	Bottom,
	Top,
	X,
	Y,
	Center,
	Color,
}

pub type Name = String;

pub enum ResolutionError {
	CannotDetermineSpecificValue,
	ConflictingConstraints,
}

#[derive(Clone)]
enum Direction {
	Left,
	Right,
	Down,
}
