use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Parse)]
pub fn derive_parse(tokens: TokenStream) -> TokenStream {
	let input = parse_macro_input!(tokens as DeriveInput);

	let enum_name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	if let Data::Enum(syn::DataEnum {
		variants: ref fields,
		..
	}) = input.data
	{
		let serial_match_branch = fields.iter().enumerate().map(|(i, f)| {
			let i = i as u8;
			let var_name = &f.ident;
			if let syn::Fields::Unit = f.fields {
				quote!( #enum_name::#var_name => bytes.push(#i), )
			} else if let syn::Fields::Unnamed(_) = f.fields {
				quote!( #enum_name::#var_name(ref val) => { bytes.push(#i); bytes.extend(val.serialise()?); }, )
			} else {
				panic!("named fields on enums not supported");
			}
		});

		let deserial_match_branch = fields.iter().enumerate().map(|(i, f)| {
			let var_name = &f.ident;
			let i = i as u8;
			if let syn::Fields::Unit = f.fields {
				quote!( #i => #enum_name::#var_name, )
			} else if let syn::Fields::Unnamed(unamed) = &f.fields {
				let var_types = &unamed.unnamed;
				if var_types.len() != 1 {
					panic!("only one field in enum variants supported currently!");
				}

				let var_type = &var_types.first().unwrap().ty;
				if let syn::Type::Path(name) = var_type {
					quote!( #i => {
                    let val;
                    (input, val) = #name::deserialise(input)?;
                    #enum_name::#var_name(val)
                } )
				} else {
					panic!("non trivial types not supported!");
				}
			} else {
				panic!("named fields on enums not supported");
			}
		});

		quote! {
			impl #impl_generics crate::parse::Parse for #enum_name #ty_generics #where_clause {
				fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
					let mut bytes = Vec::new();
					match self {
						#( #serial_match_branch )*
					}
					Ok(bytes)
				}
				fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
					let (mut input, tag) = u8::deserialise(input)?;
					let val = match tag {
						#( #deserial_match_branch )*
						_ => return Err(nom::Err::Failure(ParserError::InvalidVariant)),
					};
					Ok((input, val))
				}
			}
		}
		.into()
	} else if let Data::Struct(_) = input.data {
		panic!("#[derive(Parse)] doesn't yet work with structs!");
	} else {
		panic!("#[derive(Parse)] only works on enums!");
	}
}
