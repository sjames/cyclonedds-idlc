use crate::ast::IdlTypeSpec;

impl IdlTypeSpec {
    pub fn get_meta_op(&self, _struct_name: &str, _field_name: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_type() {
        let spec = IdlTypeSpec::I32Type;

        spec.get_meta_op("StructName", "field");
    }
}
