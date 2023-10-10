use std::marker::PhantomData;

use super::env::AsyncContext;
use crate::task::{env::Handle, Cancel, Task};

pub trait AsyncTask<Context: AsyncContext, Output> {
	fn run(self, context: Handle<Context>) -> Output;
}

pub struct BlockOn<Context: AsyncContext, T: Task<Output, C>, C: Cancel, Output> {
	task: T,
	phantom: PhantomData<(Context, C, Output)>
}

impl<Context: AsyncContext, T: Task<Output, C>, C: Cancel, Output> BlockOn<Context, T, C, Output> {
	#[inline(always)]
	pub const fn new(task: T) -> Self {
		Self { task, phantom: PhantomData }
	}
}

impl<Context: AsyncContext, T: Task<Output, C>, C: Cancel, Output> AsyncTask<Context, Output>
	for BlockOn<Context, T, C, Output>
{
	#[inline(always)]
	fn run(self, mut context: Handle<Context>) -> Output {
		context.block_on(self.task)
	}
}
