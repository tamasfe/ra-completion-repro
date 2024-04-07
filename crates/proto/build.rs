use std::{fs, path::Path};

fn main() {
    let o = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=OUT_DIR={}", o);

    fs::write(
        Path::new(&o).join("example.rs"),
        r#"
        pub mod foo {
            #[derive(Debug, Default, proto_derive::NotInScope)]
            pub struct Whatever {
                pub id: i32,
            }
        }
        "#,
    )
    .unwrap();
}
