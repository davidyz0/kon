use super::*;

pub trait Task {
	type Output;

	fn run(self, context: Handle<Context>) -> Self::Output;
}
