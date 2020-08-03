use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Item, ItemEnum, ItemStruct, Result, Token,
};

struct Args {
    traits: Vec<Ident>,
}

struct TargetItem {
    ident: Ident,
    item_impl: TargetItemImpl,
}

enum TargetItemImpl {
    Enum(ItemEnum),
    Struct(ItemStruct),
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let traits = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        if traits.is_empty() {
            Err(input.error("expected at least one trait name as an argument; none provided."))
        } else {
            Ok(Args {
                traits: traits.into_iter().collect(),
            })
        }
    }
}

impl Parse for TargetItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item = Item::parse(input)?;
        match &item {
            Item::Struct(s) => Ok(TargetItem {
                ident: s.ident.clone(),
                item_impl: TargetItemImpl::Struct(s.clone()),
            }),
            Item::Enum(e) => Ok(TargetItem {
                ident: e.ident.clone(),
                item_impl: TargetItemImpl::Enum(e.clone()),
            }),
            _ => Err(input.error("must_implement_trait can only be used on structs and enums.")),
        }
    }
}

impl ToTokens for TargetItemImpl {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match &self {
            TargetItemImpl::Struct(s) => s.to_tokens(tokens),
            TargetItemImpl::Enum(e) => e.to_tokens(tokens),
        }
    }
}

#[proc_macro_attribute]
pub fn must_implement_trait(attr_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item_tokens as TargetItem);
    let args = parse_macro_input!(attr_tokens as Args);

    let ident_str = &item.ident;
    let item_declaration = item.item_impl;
    let traits = &args.traits;

    let trait_names = args
        .traits
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>();
    let shim_trait_id = syn::Ident::new(&trait_names.join(""), Span::call_site());
    let shim_ident = format_ident!(
        "_MustImplementTraitGadget{}For{}",
        item.ident,
        shim_trait_id
    );

    // TODO: would quote_spanned! improve diagnostics?
    let updated_syntax = quote! {
        #item_declaration
        struct #shim_ident where #ident_str: #(#traits)+*;
    };

    updated_syntax.into()
}
