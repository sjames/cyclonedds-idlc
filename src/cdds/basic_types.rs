// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

/*
BOOLEAN ("bool", "DDS_OP_TYPE_BOO", "DDS_OP_SUBTYPE_BOO", Alignment.BOOL, "Boolean"),
OCTET ("uint8_t", "DDS_OP_TYPE_1BY", "DDS_OP_SUBTYPE_1BY", Alignment.ONE, "Octet"),
CHAR ("char", "DDS_OP_TYPE_1BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_1BY | DDS_OP_FLAG_SGN", Alignment.ONE, "Char"),
SHORT ("int16_t", "DDS_OP_TYPE_2BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_2BY | DDS_OP_FLAG_SGN", Alignment.TWO, "Short"),
USHORT ("uint16_t", "DDS_OP_TYPE_2BY", "DDS_OP_SUBTYPE_2BY", Alignment.TWO, "UShort"),
LONG ("int32_t", "DDS_OP_TYPE_4BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_SGN", Alignment.FOUR, "Long"),
ULONG ("uint32_t", "DDS_OP_TYPE_4BY", "DDS_OP_SUBTYPE_4BY", Alignment.FOUR, "ULong"),
LONGLONG ("int64_t", "DDS_OP_TYPE_8BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_SGN", Alignment.EIGHT, "LongLong"),
ULONGLONG ("uint64_t", "DDS_OP_TYPE_8BY", "DDS_OP_SUBTYPE_8BY", Alignment.EIGHT, "ULongLong"),
FLOAT ("float", "DDS_OP_TYPE_4BY | DDS_OP_FLAG_FP", "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_FP", Alignment.FOUR, "Float"),
DOUBLE ("double", "DDS_OP_TYPE_8BY | DDS_OP_FLAG_FP", "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_FP", Alignment.EIGHT, "Double"),
STRING ("char *", "DDS_OP_TYPE_STR", "DDS_OP_SUBTYPE_STR", Alignment.PTR, "String");
*/

use crate::cdds::alignment::{Alignment, AlignmentType};
use crate::cdds::type_trait::Type;
use crate::IdlModule;

enum BType {
    Boolean,
    Octet,
    Char,
    Short,
    UShort,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Float,
    Double,
    String,
}

pub struct BasicType {
    basic_type: BType,
    ctype: &'static str,
    op: &'static str,
    subop: &'static str,
    align: Alignment,
    xml: &'static str,
}

impl BasicType {
    pub fn new_boolean() -> Self {
        BasicType {
            basic_type: BType::Boolean,
            ctype: "bool",
            op: "DDS_OP_TYPE_BOO",
            subop: "DDS_OP_SUBTYPE_BOO",
            align: Alignment::new(AlignmentType::Bool),
            xml: "Boolean",
        }
    }
    pub fn new_octet() -> Self {
        BasicType {
            basic_type: BType::Octet,
            ctype: "uint8_t",
            op: "DDS_OP_TYPE_1BY",
            subop: "DDS_OP_SUBTYPE_1BY",
            align: Alignment::new(AlignmentType::One),
            xml: "Octet",
        }
    }
    pub fn new_char() -> Self {
        BasicType {
            basic_type: BType::Char,
            ctype: "char",
            op: "DDS_OP_TYPE_1BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_1BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::One),
            xml: "Char",
        }
    }
    pub fn new_short() -> Self {
        BasicType {
            basic_type: BType::Short,
            ctype: "int16_t",
            op: "DDS_OP_TYPE_2BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_2BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::Two),
            xml: "Short",
        }
    }
    pub fn new_ushort() -> Self {
        BasicType {
            basic_type: BType::UShort,
            ctype: "uint16_t",
            op: "DDS_OP_TYPE_2BY",
            subop: "DDS_OP_SUBTYPE_2BY",
            align: Alignment::new(AlignmentType::Two),
            xml: "UShort",
        }
    }
    pub fn new_long() -> Self {
        //LONG ("int32_t", "DDS_OP_TYPE_4BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_SGN", Alignment.FOUR, "Long"),
        BasicType {
            basic_type: BType::Long,
            ctype: "int32_t",
            op: "DDS_OP_TYPE_4BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::Four),
            xml: "Long",
        }
    }
    pub fn new_ulong() -> Self {
        //ULONG ("uint32_t", "DDS_OP_TYPE_4BY", "DDS_OP_SUBTYPE_4BY", Alignment.FOUR, "ULong"),
        BasicType {
            basic_type: BType::ULong,
            ctype: "uint32_t",
            op: "DDS_OP_TYPE_4BY",
            subop: "DDS_OP_SUBTYPE_4BY",
            align: Alignment::new(AlignmentType::Four),
            xml: "ULong",
        }
    }
    pub fn new_longlong() -> Self {
        //LONGLONG ("int64_t", "DDS_OP_TYPE_8BY | DDS_OP_FLAG_SGN", "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_SGN", Alignment.EIGHT, "LongLong"),
        BasicType {
            basic_type: BType::LongLong,
            ctype: "int64_t",
            op: "DDS_OP_TYPE_8BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::Eight),
            xml: "LongLong",
        }
    }
    pub fn new_ulonglong() -> Self {
        //ULONGLONG ("uint64_t", "DDS_OP_TYPE_8BY", "DDS_OP_SUBTYPE_8BY", Alignment.EIGHT, "ULongLong"),
        BasicType {
            basic_type: BType::ULongLong,
            ctype: "uint64_t",
            op: "DDS_OP_TYPE_8BY",
            subop: "DDS_OP_SUBTYPE_8BY",
            align: Alignment::new(AlignmentType::Eight),
            xml: "ULongLong",
        }
    }
    pub fn new_float() -> Self {
        //FLOAT ("float", "DDS_OP_TYPE_4BY | DDS_OP_FLAG_FP", "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_FP", Alignment.FOUR, "Float"),
        BasicType {
            basic_type: BType::Float,
            ctype: "float",
            op: "DDS_OP_TYPE_4BY | DDS_OP_FLAG_FP",
            subop: "DDS_OP_SUBTYPE_4BY | DDS_OP_FLAG_FP",
            align: Alignment::new(AlignmentType::Four),
            xml: "Float",
        }
    }
    pub fn new_double() -> Self {
        //DOUBLE ("double", "DDS_OP_TYPE_8BY | DDS_OP_FLAG_FP", "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_FP", Alignment.EIGHT, "Double"),
        BasicType {
            basic_type: BType::Double,
            ctype: "double",
            op: "DDS_OP_TYPE_8BY | DDS_OP_FLAG_FP",
            subop: "DDS_OP_SUBTYPE_8BY | DDS_OP_FLAG_FP",
            align: Alignment::new(AlignmentType::Eight),
            xml: "Double",
        }
    }
    pub fn new_string() -> Self {
        //STRING ("char *", "DDS_OP_TYPE_STR", "DDS_OP_SUBTYPE_STR", Alignment.PTR, "String");
        BasicType {
            basic_type: BType::String,
            ctype: "char *",
            op: "DDS_OP_TYPE_STR",
            subop: "DDS_OP_SUBTYPE_STR",
            align: Alignment::new(AlignmentType::Ptr),
            xml: "String",
        }
    }
}

impl Type for BasicType {
    fn get_meta_op(&self, name: &str, struct_name: &str, is_key: bool, root: &IdlModule) -> Vec<String> {
        vec![String::from(format!(
            "DDS_OP_ADR | {} {} , offsetof!({},{}) as u32",
            self.op,
            if is_key { " | DDS_OP_FLAG_KEY" } else { "" },
            struct_name,
            name
        ))]
    }
    /*
    fn get_meta_op_c(&self, name: &str, struct_name: &str, is_key: bool, root: &IdlModule) -> String {
        String::from(format!(
            "DDS_OP_ADR | {} {} , offsetof ({},{})",
            self.op,
            if is_key { " | DDS_OP_FLAG_KEY" } else { "" },
            struct_name,
            name
        ))
    }
    */
    fn get_sub_op(&self, root: &IdlModule) -> String {
        self.subop.into()
    }
    fn get_op(&self, root: &IdlModule) -> String {
        self.op.into()
    }

    fn get_c_type(&self, root: &IdlModule) -> String {
        self.ctype.into()
    }

    fn get_xml(&self, root: &IdlModule) -> String {
        String::from(format!("<{}/>", self.xml))
    }

    fn get_key_size(&self, root: &IdlModule) -> i32 {
        match self.basic_type {
            BType::Boolean => 1,
            BType::String => -1,
            _ => self.align.get_value(),
        }
    }

    fn get_meta_op_size(&self, root: &IdlModule) -> i32 {
        2
    }
    fn get_alignment(&self, root: &IdlModule) -> Alignment {
        self.align.clone()
    }
    fn contains_union(&self, root: &IdlModule) -> bool {
        false
    }
}
