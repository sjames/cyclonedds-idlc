mod alignment;
mod ast_c;
mod ast_h;
mod basic_types;
mod type_impl;
pub mod type_trait;

pub const INDENTION: usize = 4;

pub fn header_macro_name(stem: &str) -> String {
    String::from(format!("_DDSL_{}_H_", stem.to_uppercase()))
}

pub fn scoped_name(scope: &Vec<String>, id: &str) -> String {
    let mut name = scope.join("_");
    name.push_str("_");
    name.push_str(id);
    String::from(name)
}
