use crate::c_generator::alignment::{Alignment, AlignmentType};
use crate::c_generator::basic_types::BasicType;
use crate::c_generator::type_trait::Type;
use crate::{IdlTypeSpec, IdlScopedName};

use std::io::Error;
use std::io::Write;

use lazy_static::lazy_static;

/*
  None,
    ArrayType(Box<IdlTypeSpec>, Vec<Box<IdlValueExpr>>),
    SequenceType(Box<IdlTypeSpec>, Option<Box<IdlValueExpr>>),
    StringType(Option<Box<IdlValueExpr>>),
    WideStringType(Option<Box<IdlValueExpr>>),
    // FixedPtType,
    // EnumDcl,
    // BitsetDcl,
    // BitmaskDcl,
    F32Type,
    F64Type,
    F128Type,
    I16Type,
    I32Type,
    I64Type,
    U16Type,
    U32Type,
    U64Type,
    CharType,
    WideCharType,
    BooleanType,
    OctetType,
    // AnyType,
    // ObjectType,
    // ValueBaseType,
    ScopedName(IdlScopedName),
*/

/*
 pub fn new_boolean() -> Self {
 pub fn new_octet() -> Self {
 pub fn new_char() -> Self {
 pub fn new_short() -> Self {
 pub fn new_ushort() -> Self {
 pub fn new_long() -> Self {
 pub fn new_ulong() -> Self {
 pub fn new_longlong() -> Self {
 pub fn new_ulonglong() -> Self {
 pub fn new_float() -> Self {
 pub fn new_double() -> Self {
 pub fn new_string() -> Self {

*/

lazy_static! {
    static ref BOOLEAN: BasicType = BasicType::new_boolean();
    static ref OCTET: BasicType = BasicType::new_octet();
    static ref CHAR: BasicType = BasicType::new_char();
    static ref SHORT: BasicType = BasicType::new_short();
    static ref USHORT: BasicType = BasicType::new_ushort();
    static ref LONG: BasicType = BasicType::new_long();
    static ref ULONG: BasicType = BasicType::new_ulong();
    static ref LONGLONG: BasicType = BasicType::new_longlong();
    static ref ULONGLONG: BasicType = BasicType::new_ulonglong();
    static ref FLOAT: BasicType = BasicType::new_float();
    static ref DOUBLE: BasicType = BasicType::new_double();
    static ref STRING: BasicType = BasicType::new_string();
}

impl Type for IdlTypeSpec {
    fn get_meta_op(&self, name: &str, struct_name: &str, is_key_field: bool) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::F64Type => DOUBLE.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::I32Type => LONG.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::I64Type => LONGLONG.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::U16Type => USHORT.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::U32Type => ULONG.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::U64Type => ULONGLONG.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::CharType => CHAR.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::OctetType => OCTET.get_meta_op(name, struct_name, is_key_field),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_sub_op(&self) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => {String::from("NOT IMPLEMENTED")},
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_sub_op(),
            IdlTypeSpec::F64Type => DOUBLE.get_sub_op(),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_sub_op(),
            IdlTypeSpec::I32Type => LONG.get_sub_op(),
            IdlTypeSpec::I64Type => LONGLONG.get_sub_op(),
            IdlTypeSpec::U16Type => USHORT.get_sub_op(),
            IdlTypeSpec::U32Type => ULONG.get_sub_op(),
            IdlTypeSpec::U64Type => ULONGLONG.get_sub_op(),
            IdlTypeSpec::CharType => CHAR.get_sub_op(),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_sub_op(),
            IdlTypeSpec::OctetType => OCTET.get_sub_op(),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_op(&self) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_op(),
            IdlTypeSpec::F64Type => DOUBLE.get_op(),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_op(),
            IdlTypeSpec::I32Type => LONG.get_op(),
            IdlTypeSpec::I64Type => LONGLONG.get_op(),
            IdlTypeSpec::U16Type => USHORT.get_op(),
            IdlTypeSpec::U32Type => ULONG.get_op(),
            IdlTypeSpec::U64Type => ULONGLONG.get_op(),
            IdlTypeSpec::CharType => CHAR.get_op(),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_op(),
            IdlTypeSpec::OctetType => OCTET.get_op(),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_c_type(&self) -> String {
        match self {
            IdlTypeSpec::ArrayType(typespec, _values) => {
                // The array sizes are handled at the StructMember. 
                typespec.get_c_type()
            },
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("dds_sequence_t"),
            IdlTypeSpec::StringType(_value) => STRING.get_c_type(),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_c_type(),
            IdlTypeSpec::F64Type => DOUBLE.get_c_type(),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_c_type(),
            IdlTypeSpec::I32Type => LONG.get_c_type(),
            IdlTypeSpec::I64Type => LONGLONG.get_c_type(),
            IdlTypeSpec::U16Type => USHORT.get_c_type(),
            IdlTypeSpec::U32Type => ULONG.get_c_type(),
            IdlTypeSpec::U64Type => ULONGLONG.get_c_type(),
            IdlTypeSpec::CharType => CHAR.get_c_type(),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_c_type(),
            IdlTypeSpec::OctetType => OCTET.get_c_type(),
            IdlTypeSpec::ScopedName(name) => name.get_c_type(),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_xml(&self) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_xml(),
            IdlTypeSpec::F64Type => DOUBLE.get_xml(),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_xml(),
            IdlTypeSpec::I32Type => LONG.get_xml(),
            IdlTypeSpec::I64Type => LONGLONG.get_xml(),
            IdlTypeSpec::U16Type => USHORT.get_xml(),
            IdlTypeSpec::U32Type => ULONG.get_xml(),
            IdlTypeSpec::U64Type => ULONGLONG.get_xml(),
            IdlTypeSpec::CharType => CHAR.get_xml(),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_xml(),
            IdlTypeSpec::OctetType => OCTET.get_xml(),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }

    fn get_key_size(&self) -> i32 {
        0
    }
    fn get_meta_op_size(&self) -> i32 {
        0
    }
    fn get_alignment(&self) -> Alignment {
        Alignment::new(AlignmentType::Ptr)
    }

    fn contains_union(&self) -> bool {
        false
    }
}

impl IdlTypeSpec {
    pub fn write_h<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        out.write(self.get_c_type().as_bytes())?;

        Ok(())
    }
}

impl Type for IdlScopedName {

    fn get_meta_op(&self, name: &str, struct_name: &str, is_key_field: bool) -> String {
        panic!("Unimplemented");
    }
    fn get_sub_op(&self) -> String  {
        panic!("Unimplemented");
    }
    fn get_op(&self) -> String {
        panic!("Unimplemented");
    }
    fn get_c_type(&self) -> String {
        let is_absolute_path = self.1;
        let components = &self.0;
        if is_absolute_path {
            components.join("_")
        } else {
            String::from(components.iter().last().as_deref().unwrap())
        }

    }
    fn get_xml(&self) -> String {
        panic!("Unimplemented");
    }
    fn get_key_size(&self) -> i32 {
        panic!("Unimplemented");
    }
    fn get_meta_op_size(&self) -> i32 {
        panic!("Unimplemented");
    }
    fn get_alignment(&self) -> Alignment {
        panic!("Unimplemented");
    }
    fn contains_union(&self) -> bool {
        panic!("Unimplemented");
    }
    /*
    pub fn write_h<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let is_absolute_path = self.1;

        let components = &self.0;

        let name = components.join("_");
        let _ = write!(out, "{}", name);
/*
        for (idx, comp) in components.iter().enumerate() {
            // TODO, use paths according to "crate::" or "super::"
            if idx == 0 && !is_absolute_path {
                let _ = write!(out, "{}", comp);
            } else if idx == 0 && is_absolute_path {
                let _ = write!(out, "crate::{}", comp);
            } else {
                let _ = write!(out, "::{}", comp);
            }
        }
        */
        Ok(())

    }
    */
}
