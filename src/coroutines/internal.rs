use std::marker::PhantomData;

use super::*;

#[doc(hidden)]
pub fn as_task<T, Output>(task: T) -> impl for<'ctx> Task<Output<'ctx> = Output>
where
	T: for<'ctx> Task<Output<'ctx> = Output>
{
	task
}

#[cfg(not(any(doc, feature = "xx-doc")))]
#[inline(always)]
pub fn unsafe_stub_do_not_use<T>(context: &Context, task: T) -> T::Output<'_>
where
	T: Task
{
	/* Safety: this function is a stub generated by macros only in an async
	 * function. it is an error to call this function from anywhere else
	 */
	unsafe { context.run(task) }
}

#[doc(hidden)]
#[asynchronous]
#[lang = "task_wrap"]
pub struct OpaqueTask<F, Output>(F, PhantomData<Output>);

#[doc(hidden)]
#[asynchronous]
#[lang = "async_closure"]
pub struct OpaqueAsyncFn<F, const T: usize>(F);

#[cfg(any(doc, feature = "xx-doc"))]
/// Additional bound for async traits only while generating documentation
///
/// Async traits generated by the [`asynchronous`] macro are usually object
/// safe, but the docs may say otherwise
pub trait DocDynSafe: Sized {}

#[cfg(any(doc, feature = "xx-doc"))]
#[doc(hidden)]
impl<T> DocDynSafe for T {}

#[cfg(any(doc, feature = "xx-doc"))]
#[doc(hidden)]
impl<T: std::future::Future> Task for T {
	type Output<'ctx> = T::Output;

	async fn run(self) -> T::Output {
		unreachable!();
	}
}

#[cfg(any(doc, feature = "xx-doc"))]
#[doc(hidden)]
pub struct DocAsyncFn<Args, Output>(std::marker::PhantomData<(Args, Output)>);

#[cfg(any(doc, feature = "xx-doc"))]
const _: () = {
	impl<Args, Output> DocAsyncFn<Args, Output> {
		pub(crate) fn new() -> Self {
			unreachable!();
		}
	}

	impl<Args, Output> AsyncFnOnce<Args> for DocAsyncFn<Args, Output> {
		type Output = Output;

		async fn call_once(self, args: Args) -> Output {
			unreachable!();
		}
	}

	impl<Args, Output> AsyncFnMut<Args> for DocAsyncFn<Args, Output> {
		type Output = Output;

		async fn call_mut(&mut self, args: Args) -> Output {
			unreachable!();
		}
	}

	impl<Args, Output> AsyncFn<Args> for DocAsyncFn<Args, Output> {
		type Output = Output;

		async fn call(&self, args: Args) -> Output {
			unreachable!();
		}
	}
};
