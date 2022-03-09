use anyhow::Result;
use dart::class;
use syn::{ItemStruct, Type};
use quote::quote;

mod dart;

fn main() -> Result<()> {
    let structs = [
        "pub struct RainbowInfo { pub count: u8, pub hour: u8 }",
        "pub struct Random {
            a: Wrapping<u32>,
            b: Wrapping<u32>,
            c: Wrapping<u32>,
            d: Wrapping<u32>
        }",
        "#[wasm_bindgen]
        pub struct SpecialCloudInfo {
            pub cloud: SpecialCloud,
            // TODO: not working?
            //#[wasm_bindgen(js_name = rangeStart)]
            pub range_start: u8,
            //#[wasm_bindgen(js_name = rangeEnd)]
            pub range_end: u8
        }",
    ];

    for code in structs {
        let struct_ = syn::parse_str::<ItemStruct>(code)?;
        let class = struct_to_dart(&struct_);

        println!("{class}");
    }

    println!("{}", include_str!("../predefined.dart"));

    Ok(())
}

fn struct_to_dart(struct_: &ItemStruct) -> dart::Class {
    let class_name = dart::Ident(format!("{}", &struct_.ident));
    let mut class = dart::Class::new(class_name.clone());
    let mut constructor_args = class::FnArgs::new();

    for field in &struct_.fields {
        let field_name = dart::Ident(field.ident.as_ref().map(|ident| format!("{ident}")).expect("tuple structs are not supported yet"));
        let field_type = type_to_dart(&field.ty);

        class.variables.push(class::Variable {
            name: field_name.clone(),
            ty: field_type.clone(),
        });

        constructor_args.named.push(class::NamedFnArg {
            name: field_name.clone(),
            ty: field_type,
            required: true,
            initializing: true,
        });
    }

    class.constructors.push(class::Method {
        class_name: None,
        name: class_name,
        is_factory: false,
        fn_args: constructor_args,
        body: None,
    });

    class
}

fn type_to_dart(ty: &Type) -> dart::Type {
    match ty {
        Type::Array(_) => todo!("unhandled type: Array"),
        Type::BareFn(_) => todo!("unhandled type: BareFn"),
        Type::Group(_) => todo!("unhandled type: Group"),
        Type::ImplTrait(_) => todo!("unhandled type: ImplTrait"),
        Type::Infer(_) => todo!("unhandled type: Infer"),
        Type::Macro(_) => todo!("unhandled type: Macro"),
        Type::Never(_) => todo!("unhandled type: Never"),
        Type::Paren(_) => todo!("unhandled type: Paren"),
        Type::Path(type_path) => {
            let path = &type_path.path;

            if type_path.qself.is_some() {
                panic!("self:: not supported");
            }

            if path.leading_colon.is_some() {
                panic!("leading :: not supported");
            }

            if path.segments.len() > 1 {
                panic!("type paths with more than one segment not supported");
            }

            let segment = &path.segments[0];
            let arguments = match &segment.arguments {
                syn::PathArguments::None => dart::r#type::Arguments::None,
                syn::PathArguments::AngleBracketed(arguments) => {
                    let generic_arguments = arguments.args.iter()
                        .map(|arg| match arg {
                            syn::GenericArgument::Type(arg) => type_to_dart(arg),
                            _ => todo!("not supported: {}", quote!{ #arg }),

                        })
                    .collect::<Vec<_>>();

                    dart::r#type::Arguments::Generic(generic_arguments)
                },
                syn::PathArguments::Parenthesized(_) => todo!("parenthesized type parameters"),
            };

            dart::Type{
                name: dart::Ident(segment.ident.to_string()),
                arguments,
                import: None,
            }
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
