use std::fs::read_to_string;
use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

use serde::Deserialize;
use toml::from_str;

struct Config {
    cargo: Cargo,
    dioxus: Dioxus,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "expected no input"));
        }

        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap();

        let cargo_file = cargo_manifest_dir.join("Cargo.toml");
        let dioxus_file = cargo_manifest_dir.join("Dioxus.toml");

        let cargo_contents = read_to_string(cargo_file).expect("Failed to read Cargo.toml");
        let dioxus_contents = read_to_string(dioxus_file).expect("Failed to read Dioxus.toml");

        let cargo = from_str(&cargo_contents).expect("Failed to parse Cargo.toml");
        let dioxus = from_str(&dioxus_contents).expect("Failed to parse Dioxus.toml");

        Ok(Config { cargo, dioxus })
    }
}

impl ToTokens for Config {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.dioxus.application.name.as_str();
        let version = self.cargo.package.version.as_str();
        let identifier = self.dioxus.bundle.identifier.as_str();

        tokens.extend(quote! {
            Config {
                name: #name,
                version: #version,
                identifier: #identifier,
            }
        });
    }
}

#[derive(Deserialize)]
struct Cargo {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    version: String,
}

#[derive(Deserialize)]
struct Dioxus {
    application: Application,
    bundle: Bundle,
}

#[derive(Deserialize)]
struct Application {
    name: String,
}

#[derive(Deserialize)]
struct Bundle {
    identifier: String,
}

/// Reads Cargo.toml and Dioxus.toml config files and generates a `Config` based on the contents.
#[proc_macro]
pub fn get_config(items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let config = parse_macro_input!(items as Config);
    quote! { #config }.into()
}
