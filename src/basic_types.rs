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

use crate::alignment::{Alignment, AlignmentType};

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

pub struct BT {
    basic_type: BType,
    ctype: &'static str,
    op: &'static str,
    subop: &'static str,
    align: Alignment,
    xml: &'static str,
}

impl BT {
    pub fn new_boolean() -> Self {
        BT {
            basic_type: BType::Boolean,
            ctype: "bool",
            op: "DDS_OP_TYPE_BOO",
            subop: "DDS_OP_SUBTYPE_BOO",
            align: Alignment::new(AlignmentType::Bool),
            xml: "Boolean",
        }
    }
    pub fn new_octet() -> Self {
        BT {
            basic_type: BType::Octet,
            ctype: "uint8_t",
            op: "DDS_OP_TYPE_1BY",
            subop: "DDS_OP_SUBTYPE_1BY",
            align: Alignment::new(AlignmentType::One),
            xml: "Octet",
        }
    }
    pub fn new_char() -> Self {
        BT {
            basic_type: BType::Char,
            ctype: "char",
            op: "DDS_OP_TYPE_1BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_1BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::One),
            xml: "Char",
        }
    }
    pub fn new_short() -> Self {
        BT {
            basic_type: BType::Short,
            ctype: "int16_t",
            op: "DDS_OP_TYPE_2BY | DDS_OP_FLAG_SGN",
            subop: "DDS_OP_SUBTYPE_2BY | DDS_OP_FLAG_SGN",
            align: Alignment::new(AlignmentType::Two),
            xml: "Short",
        }
    }
    pub fn new_ushort() -> Self {
        BT {
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
        BT {
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
        BT {
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
        BT {
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
        BT {
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
        BT {
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
        BT {
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
        BT {
            basic_type: BType::String,
            ctype: "char *",
            op: "DDS_OP_TYPE_STR",
            subop: "DDS_OP_SUBTYPE_STR",
            align: Alignment::new(AlignmentType::Ptr),
            xml: "String",
        }
    }
}
