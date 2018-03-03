#[macro_use] extern crate quote;
extern crate syn;

use syn::{Attribute, Item, ItemFn, ItemMod, Lit, Meta, MetaNameValue};
use std::env;
use std::io::{Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};

fn any_attr_is(attrs: &[Attribute], ident: &str) -> bool {
    attrs.iter().any(|a| match a.interpret_meta() {
        Some(Meta::Word(i)) if i == ident => true,
        _ => false
    })
}

fn parse(mod_path: PathBuf, items: Vec<Item>) -> Vec<syn::Path> {
    let mut names = vec![];

    for item in items {
        match item {
            Item::Fn(ItemFn { ref attrs, ident, .. })
                if any_attr_is(attrs, "register") => {
                    names.push(ident.into());
                }

            Item::Mod(module) => {
                let (the_path, the_items, the_ident);
                match module {
                    ItemMod { content: Some((_, items)), ident, .. } => {
                        the_items = items;
                        the_ident = ident;
                        the_path = mod_path.clone();
                    }
                    ItemMod { attrs, ident, .. } => {
                        let mut path = None;
                        for attr in attrs {
                            match attr.interpret_meta() {
                                Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(ref s), .. }))
                                    if ident == "path" => {
                                        path = Some(s.value());
                                    }

                                _ => {}
                            }
                        }

                        let mut content = String::new();
                        let mut file = match path {
                            Some(p) => {
                                the_path = Path::new(&p).parent().unwrap().to_owned();
                                File::open(&p).expect(&p)
                            }
                            
                            None => {
                                match File::open(mod_path.join(format!("{}.rs", ident))) {
                                    Ok(file) => {
                                        the_path = mod_path.clone();
                                        file
                                    }
                                    Err(_) => {
                                        the_path = mod_path.join(ident.as_ref());
                                        File::open(mod_path.join(ident.as_ref()).join("mod.rs")).expect(&format!("{}/{}/mod.rs", mod_path.display(), ident))
                                    }
                                }
                            }
                        };
                        file.read_to_string(&mut content).unwrap();
                        the_items = syn::parse_file(&content).unwrap().items;
                        the_ident = ident;
                    }
                }

                names.extend(
                    parse(the_path, the_items)
                        .into_iter()
                        .map(|mut p| {
                            p.segments.insert(0, the_ident.into());
                            p
                        })
                    );
            }

            _ => {}
        }
    }

    names
}

pub fn go<P: AsRef<Path>>(root: P) {
    let root = root.as_ref();
    let outfile = Path::new(&env::var("OUT_DIR").unwrap()).join("macbuild.rs");

    println!("cargo:rustc-env=MACBUILD={}", outfile.display());
    
    let mut content = String::new();
    File::open(root).unwrap().read_to_string(&mut content).unwrap();
    let ast = syn::parse_file(&content).unwrap();
    let names = parse(root.parent().unwrap().to_owned(), ast.items);

    let mut out = File::create(outfile).unwrap();
    writeln!(out, "{}", quote! {
        pub fn bootstrap() {
            #(::#names();)*
        }
    }).unwrap();
}

