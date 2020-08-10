// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

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
