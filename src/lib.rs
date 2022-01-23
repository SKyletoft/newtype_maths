use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

macro_rules! generate_bin_ops_derive {
	($name:ident, $Op:ident, $op:ident, $target:ty) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let ident = parse_macro_input!(item as DeriveInput).ident;
			let example = quote!(
				impl ::core::ops::$Op<$target> for #ident {
					type Output = Self;
					fn $op(self, rhs: Self) -> Self::Output {
						Self(self.0.$op(rhs.0))
					}
				}
			);
			TokenStream::from(example)
		}
	};
}

macro_rules! generate_assign_ops_derive {
	($name:ident, $Op:ident, $op:ident, $target:ty) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let ident = parse_macro_input!(item as DeriveInput).ident;
			let example = quote!(
				impl ::core::ops::$Op<$target> for #ident {
					fn $op(&mut self, rhs: Self) {
						self.0.$op(rhs.0);
					}
				}
			);
			TokenStream::from(example)
		}
	};
}

macro_rules! generate_un_ops_derive {
	($name:ident, $Op:ident, $op:ident) => {
		#[proc_macro_derive($Op)]
		pub fn $name(item: TokenStream) -> TokenStream {
			let ident = parse_macro_input!(item as DeriveInput).ident;
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
generate_assign_ops_derive!(derive_bit_and_assign_trait, BitAndAssign, bit_and_assign, Self);
generate_assign_ops_derive!(derive_bit_or_assign_trait, BitOrAssign, bit_or_assign, Self);
generate_assign_ops_derive!(derive_bit_xor_assign_trait, BitXorAssign, bit_xor_assign, Self);
generate_assign_ops_derive!(derive_rem_assign_trait, RemAssign, rem_assign, Self);
generate_assign_ops_derive!(derive_shl_assign_trait, ShlAssign, shl_assign, Self);
generate_assign_ops_derive!(derive_shr_assign_trait, ShrAssign, shr_assign, Self);

generate_un_ops_derive!(derive_neg_trait, Neg, neg);
generate_un_ops_derive!(derive_not_trait, Not, not);

