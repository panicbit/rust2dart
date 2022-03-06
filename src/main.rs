use std::fmt::Write;

use anyhow::Result;
use itertools::Itertools;
use syn::{ItemStruct, Type};
use quote::{quote, format_ident};

mod dart;

fn main() -> Result<()> {
    let structs = [
        "pub struct RainbowInfo { pub count: u8, pub hour: u8 }",

    ];

    for code in structs {
        let struct_ = syn::parse_str::<ItemStruct>(code)?;

        let mut out = String::new();

        struct_to_dart(&struct_, &mut out);

        println!("{out}");
    }

    Ok(())
}

fn struct_to_dart(struct_: &ItemStruct, out: &mut String) {
    let name = format!("{}", &struct_.ident);
    let fields = struct_.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("tuple structs are not supported yet");
        let field_type = type_to_dart(&field.ty);

        format!("    {field_type} {field_name};")
    })
    .join("\n");
    let constructor_parameters = struct_.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("tuple structs are not supported yet");

        format!("        required this.{field_name}")
    })
    .join(",\n");

    write!(out, r#"
class {name} {{
{fields}
    {name}({{
{constructor_parameters}
    }});
}}
    "#)
    .unwrap();
}

fn type_to_dart(ty: &Type) -> String {
    match ty {
        Type::Array(_) => todo!("unhandled type: Array"),
        Type::BareFn(_) => todo!("unhandled type: BareFn"),
        Type::Group(_) => todo!("unhandled type: Group"),
        Type::ImplTrait(_) => todo!("unhandled type: ImplTrait"),
        Type::Infer(_) => todo!("unhandled type: Infer"),
        Type::Macro(_) => todo!("unhandled type: Macro"),
        Type::Never(_) => todo!("unhandled type: Never"),
        Type::Paren(_) => todo!("unhandled type: Paren"),
        Type::Path(path) => {
            let ident = path.path.get_ident().expect("no path ident?");

            ident.to_string()
        }
        Type::Ptr(_) => todo!("unhandled type: Ptr"),
        Type::Reference(_) => todo!("unhandled type: Reference"),
        Type::Slice(_) => todo!("unhandled type: Slice"),
        Type::TraitObject(_) => todo!("unhandled type: TraitObject"),
        Type::Tuple(_) => todo!("unhandled type: Tuple"),
        Type::Verbatim(_) => todo!("unhandled type: Verbatim"),
        _ => todo!("unhandled type: odo"),
    }
}
