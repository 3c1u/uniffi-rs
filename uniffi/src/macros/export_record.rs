/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use std::convert::{Into, TryFrom};
use syn::spanned::Spanned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDefinition {}

impl RecordDefinition {}

impl TryFrom<&syn::ItemStruct> for RecordDefinition {
    type Error = syn::Error;
    fn try_from(_item: &syn::ItemStruct) -> syn::Result<Self> {
        Ok(RecordDefinition {})
    }
}

impl Into<proc_macro2::TokenStream> for &RecordDefinition {
    fn into(self) -> proc_macro2::TokenStream {
        quote! {
            // Not implemented yet...
        }
    }
}

impl quote::ToTokens for RecordDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tt: proc_macro2::TokenStream = self.into();
        tt.to_tokens(tokens);
    }
}
