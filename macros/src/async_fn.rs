use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, visit_mut::*, *};

use crate::{closure::into_basic_closure, transform::transform_fn};

struct ReplaceAwait;

impl VisitMut for ReplaceAwait {
	fn visit_expr_mut(&mut self, expr: &mut Expr) {
		visit_expr_mut(self, expr);

		if let Expr::Await(inner) = expr {
			let base = inner.base.as_ref();
			let mut call: ExprCall = parse_quote! {
				xx_core::coroutines::env::AsyncContext::run(__xx_internal_async_context.clone().as_mut(), #base)
			};

			call.attrs = inner.attrs.clone();

			*expr = Expr::Call(call);
		}
	}
}

fn transform_with_type(
	is_item_fn: bool, attrs: &mut Vec<Attribute>, sig: &mut Signature, block: Option<&mut Block>,
	context_type: proc_macro2::TokenStream, make_closure: bool
) -> Result<()> {
	if sig.asyncness.take().is_none() {
		if !is_item_fn {
			return Ok(());
		}

		return Err(Error::new(
			sig.span(),
			"The `async` keyword is missing from the function declaration"
		));
	}

	attrs.push(parse_quote!( #[must_use = "Future does nothing until you `.await` it"] ));

	let block = if let Some(block) = block {
		ReplaceAwait {}.visit_block_mut(block);
		Some(block)
	} else {
		None
	};

	if make_closure {
		into_basic_closure(
			attrs,
			sig,
			block,
			vec![quote! { mut __xx_internal_async_context }],
			vec![quote! { xx_core::task::env::Handle<#context_type> }],
			|rt| rt,
			Some(|rt| {
				(
					quote! { xx_core::coroutines::task::AsyncTask<#context_type, #rt> },
					quote! { xx_core::coroutines::closure::AsyncClosure }
				)
			})
		)?;
	} else {
		let mutability = if block.is_some() {
			quote! { mut }
		} else {
			quote! {}
		};

		sig.inputs.push(
			parse_quote! { #mutability __xx_internal_async_context: xx_core::task::env::Handle<#context_type> }
		);
	}

	Ok(())
}

fn transform_generic(
	is_item_fn: bool, attrs: &mut Vec<Attribute>, sig: &mut Signature, block: Option<&mut Block>
) -> Result<()> {
	transform_with_type(is_item_fn, attrs, sig, block, quote! { Context }, true)?;

	Ok(())
}

fn transform_typed(
	is_item_fn: bool, attrs: &mut Vec<Attribute>, sig: &mut Signature, block: Option<&mut Block>
) -> Result<()> {
	transform_with_type(
		is_item_fn,
		attrs,
		sig,
		block,
		quote! { xx_async_runtime::Context },
		true
	)?;

	Ok(())
}

fn transform_generic_no_closure(
	is_item_fn: bool, attrs: &mut Vec<Attribute>, sig: &mut Signature, block: Option<&mut Block>
) -> Result<()> {
	transform_with_type(is_item_fn, attrs, sig, block, quote! { Context }, false)?;

	Ok(())
}

/// ### Input
/// ```
/// #[async_fn]
/// async fn async_add(&mut self, a: i32, b: i32) -> i32 {
/// 	a + b
/// }
/// ```
///
/// ### Output
/// ```
/// fn async_add(&mut self, a: i32, b: i32) ->
/// 	AsyncClosure<(&mut Self, i32, i32), i32> {
/// 	let run = |
/// 		(__self, a, b): (&mut Self, i32, i32),
/// 		context: AsyncContext
/// 	| -> i32 {
/// 		a + b
/// 	}
///
/// 	AsyncClosure::new((self, a, b), run)
/// }
/// ```
pub fn async_fn(_: TokenStream, item: TokenStream) -> TokenStream {
	match transform_fn(item, transform_generic) {
		Ok(ts) => ts,
		Err(err) => err.to_compile_error().into()
	}
}

pub fn async_fn_typed(_: TokenStream, item: TokenStream) -> TokenStream {
	match transform_fn(item, transform_typed) {
		Ok(ts) => ts,
		Err(err) => err.to_compile_error().into()
	}
}

pub fn async_fn_no_closure(_: TokenStream, item: TokenStream) -> TokenStream {
	match transform_fn(item, transform_generic_no_closure) {
		Ok(ts) => ts,
		Err(err) => err.to_compile_error().into()
	}
}
