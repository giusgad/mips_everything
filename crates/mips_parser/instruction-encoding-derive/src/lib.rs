use std::collections::HashMap;
use proc_macro::TokenStream;
use syn::{token::Comma, Data, DeriveInput, Ident, LitInt};

#[derive(Debug)]
struct VariantAttrs {
    opcode: u8,
    format: Ident,
    funct: Option<u8>,
}

impl syn::parse::Parse for VariantAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let opcode = input.parse::<LitInt>()?.base10_parse::<u8>()?;
        input.parse::<Comma>()?;
        let format = input.parse::<Ident>()?;
        if !(format == "R" || format == "J" || format == "I") {
            return Err(syn::Error::new(
                format.span(),
                "Invalid variant for instruction format.",
            ));
        }
        let funct = if input.peek(Comma) {
            input.parse::<Comma>()?;
            let funct = input.parse::<LitInt>()?.base10_parse::<u8>()?;
            if format != "R" {
                return Err(syn::Error::new(
                    input.span(),
                    "Funct must not be specified for non R instructions.",
                ));
            }
            Some(funct)
        } else {
            // there was no funct
            if format == "R" {
                return Err(syn::Error::new(
                    input.span(),
                    "Required funct for R instructions.",
                ));
            }
            None
        };
        Ok(VariantAttrs {
            opcode,
            format,
            funct,
        })
    }
}

fn impl_encoding_trait(ast: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(data) = ast.data else {
        panic!("This derive macro only works on enums.")
    };
    let enum_ident = ast.ident;
    let mut attrs: HashMap<Ident, VariantAttrs> = HashMap::new();

    for variant in &data.variants {
        let mut found_attr = false;
        for attr in &variant.attrs {
            if let Some(attr_meta_name) = attr.path().get_ident() {
                if attr_meta_name == "instruction" {
                    found_attr = true;
                    let attr = attr.parse_args::<VariantAttrs>()?;
                    attrs.insert(variant.ident.clone(), attr);
                }
            }
        }
        if !found_attr {
            return Err(syn::Error::new(
                enum_ident.span(),
                "`instruction` attribute missing on this variant",
            ));
        }
    }

    let format_match = attrs
        .iter()
        .map(|(k, v)| {
            let format = v.format.clone();
            quote::quote! {#enum_ident::#k => InstructionFormat::#format}
        })
        .collect::<Vec<_>>();
    let funct_match = attrs
        .iter()
        .map(|(k, v)| {
            if let Some(funct) = v.funct {
                quote::quote! {#enum_ident::#k => Some(Bits::new(#funct as u32))}
            } else {
                quote::quote! {#enum_ident::#k => None}
            }
        })
        .collect::<Vec<_>>();
    let opcode_match = attrs
        .into_iter()
        .map(|(k, v)| {
            let opcode = v.opcode;
            quote::quote! {#enum_ident::#k => Bits::new(#opcode as u32)}
        })
        .collect::<Vec<_>>();

    Ok(quote::quote! {
        impl InstructionEncoding for #enum_ident {
            fn format(&self) -> InstructionFormat {
                match self {#(#format_match),*}
            }
            fn opcode(&self) -> Bits<6> {
                match self {#(#opcode_match),*}
            }
            fn funct(&self) -> Option<Bits<6>> {
                match self {#(#funct_match),*}
            }
        }
    }
    .into())
}

/// This macro implements the [`mips_parser::defs::InstructionEncoding`] trait on an enum.
/// Each variant must have an `#[instruction(opcode, format, funct)]` attribute.
/// funct must only be specified if format is `R`.
/// # Example
/// ```ignore
/// #[derive(InstructionEncoding)]
/// enum Instruction {
///     #[instruction(0b010010, R, 0b100000)]
///     Add,
///     #[instruction(0b010011, I)]
///     Addi,
/// }
/// ```
#[proc_macro_derive(InstructionEncoding, attributes(instruction))]
pub fn instruction_encoding_derive_macro(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    match impl_encoding_trait(ast) {
        Ok(ts) => ts,
        Err(err) => err.into_compile_error().into(),
    }
}
