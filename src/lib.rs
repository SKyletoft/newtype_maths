use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Horrible unhygienic trick to error out early
macro_rules! type_or_error {
	($data:ident, $ident:ident) => {{
		let error = syn::Error::new(
			$ident.span(),
			"only single field tuple structs are supported",
		)
		.to_compile_error()
		.into();

		let unnamed = if let syn::Data::Struct(syn::DataStruct {
			fields: syn::Fields::Unnamed(syn::FieldsUnnamed { ref unnamed, .. }),
			..
		}) = $data
		{
			unnamed
		} else {
			return error;
		};
		if unnamed.len() != 1 {
			return error;
		}
		let inner_type = if let Some(field) = unnamed.first() {
			&field.ty
		} else {
			return error;
		};

		inner_type
	}};
}

/// $Op and $op are expected to be the same name, but one's capitalised
macro_rules! generate_bin_ops_derive {
	($name:ident, $Op:ident, $op:ident, $target:ty) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let DeriveInput {
				attrs: _,
				vis: _,
				ident,
				generics: _,
				data,
			} = parse_macro_input!(item as DeriveInput);
			type_or_error!(data, ident);

			let example = quote!(
				impl ::core::ops::$Op<$target> for #ident {
					type Output = Self;
					fn $op(self, rhs: $target) -> Self::Output {
						Self(self.0.$op(rhs.0))
					}
				}
			);
			TokenStream::from(example)
		}
	};
}

/// $Op and $op are expected to be the same name, but one's capitalised
macro_rules! generate_assign_ops_derive {
	($name:ident, $Op:ident, $op:ident, $target:ty) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let DeriveInput {
				attrs: _,
				vis: _,
				ident,
				generics: _,
				data,
			} = parse_macro_input!(item as DeriveInput);
			type_or_error!(data, ident);

			let example = quote!(
				impl ::core::ops::$Op<$target> for #ident {
					fn $op(&mut self, rhs: $target) {
						self.0.$op(rhs.0);
					}
				}
			);
			TokenStream::from(example)
		}
	};
}

/// $Op and $op are expected to be the same name, but one's capitalised
macro_rules! generate_un_ops_derive {
	($name:ident, $Op:ident, $op:ident) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let DeriveInput {
				attrs: _,
				vis: _,
				ident,
				generics: _,
				data,
			} = parse_macro_input!(item as DeriveInput);
			type_or_error!(data, ident);

			let example = quote!(
				impl ::core::ops::$Op for #ident {
					type Output = Self;
					fn $op(self) -> Self::Output {
						Self(self.0.$op())
					}
				}
			);
			TokenStream::from(example)
		}
	};
}

generate_bin_ops_derive!(derive_add_trait, Add, add, Self);
generate_bin_ops_derive!(derive_sub_trait, Sub, sub, Self);
generate_bin_ops_derive!(derive_mul_trait, Mul, mul, Self);
generate_bin_ops_derive!(derive_div_trait, Div, div, Self);
generate_bin_ops_derive!(derive_bit_and_trait, BitAnd, bit_and, Self);
generate_bin_ops_derive!(derive_bit_or_trait, BitOr, bit_or, Self);
generate_bin_ops_derive!(derive_bit_xor_trait, BitXor, bit_xor, Self);
generate_bin_ops_derive!(derive_rem_trait, Rem, rem, Self);
generate_bin_ops_derive!(derive_shl_trait, Shl, shl, Self);
generate_bin_ops_derive!(derive_shr_trait, Shr, shr, Self);

generate_assign_ops_derive!(derive_add_assign_trait, AddAssign, add_assign, Self);
generate_assign_ops_derive!(derive_sub_assign_trait, SubAssign, sub_assign, Self);
generate_assign_ops_derive!(derive_mul_assign_trait, MulAssign, mul_assign, Self);
generate_assign_ops_derive!(derive_div_assign_trait, DivAssign, div_assign, Self);
generate_assign_ops_derive!(
	derive_bit_and_assign_trait,
	BitAndAssign,
	bit_and_assign,
	Self
);
generate_assign_ops_derive!(derive_bit_or_assign_trait, BitOrAssign, bit_or_assign, Self);
generate_assign_ops_derive!(
	derive_bit_xor_assign_trait,
	BitXorAssign,
	bit_xor_assign,
	Self
);
generate_assign_ops_derive!(derive_rem_assign_trait, RemAssign, rem_assign, Self);
generate_assign_ops_derive!(derive_shl_assign_trait, ShlAssign, shl_assign, Self);
generate_assign_ops_derive!(derive_shr_assign_trait, ShrAssign, shr_assign, Self);

generate_un_ops_derive!(derive_neg_trait, Neg, neg);
generate_un_ops_derive!(derive_not_trait, Not, not);

#[proc_macro_derive(Index)]
pub fn derive_index_trait(item: TokenStream) -> TokenStream {
	let DeriveInput {
		attrs: _,
		vis: _,
		ident,
		generics: _,
		data,
	} = parse_macro_input!(item as DeriveInput);
	let inner_type = type_or_error!(data, ident);

	let example = quote!(
		impl ::core::ops::Index<usize> for #ident {
			type Output = <#inner_type as ::core::ops::Index<usize>>::Output;
			fn index(&self, idx: usize) -> &Self::Output {
				self.0.index(idx)
			}
		}
	);

	TokenStream::from(example)
}

#[proc_macro_derive(IndexMut)]
pub fn derive_index_mut_trait(item: TokenStream) -> TokenStream {
	let DeriveInput {
		attrs: _,
		vis: _,
		ident,
		generics: _,
		data,
	} = parse_macro_input!(item as DeriveInput);
	type_or_error!(data, ident);

	let example = quote!(
		impl ::core::ops::IndexMut<usize> for #ident {
			fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
				self.0.index_mut(idx)
			}
		}
	);

	TokenStream::from(example)
}

#[proc_macro_derive(From)]
pub fn derive_from_trait(item: TokenStream) -> TokenStream {
	let DeriveInput {
		attrs: _,
		vis: _,
		ident,
		generics: _,
		data,
	} = parse_macro_input!(item as DeriveInput);
	let inner_type = type_or_error!(data, ident);

	let example = quote!(
		impl ::core::convert::From<#inner_type> for #ident {
			fn from(other: #inner_type) -> Self {
				Self(other)
			}
		}
	);

	TokenStream::from(example)
}
