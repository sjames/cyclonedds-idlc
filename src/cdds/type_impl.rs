// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

use crate::cdds::alignment::{Alignment, AlignmentType};
use crate::cdds::basic_types::BasicType;
use crate::cdds::type_trait::Type;
use crate::{IdlModule, IdlScopedName, IdlTypeDcl,IdlTypeDclKind, IdlTypeSpec};

use std::io::Error;
use std::io::Write;
use std::ops::Deref;

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

fn get_meta_op_complex(
    spec: &IdlTypeSpec,
    name: &str,
    struct_name: &str,
    is_key: bool,
    root: &IdlModule,
) -> Vec<String> {
    match spec {
        IdlTypeSpec::SequenceType(typespec, values) => vec![String::from(format!(
            "DDS_OP_ADR | DDS_OP_TYPE_SEQ | {}{} ,offsetof!({},{}) as u32",
            typespec.get_sub_op(root),
            if is_key { " | DDS_OP_FLAG_KEY" } else { "" },
            struct_name,
            name
        ))],
        IdlTypeSpec::ArrayType(typespec, values) => vec![String::from(format!(
            "DDS_OP_ADR | DDS_OP_TYPE_ARR | {}{} ,offsetof!({},{}) as u32",
            typespec.get_sub_op(root),
            if is_key { " | DDS_OP_FLAG_KEY" } else { "" },
            struct_name,
            name
        ))],
        _ => vec![String::from("UNIMPLEMENTED")],
    }
}

impl Type for IdlTypeSpec {
    fn get_meta_op(
        &self,
        name: &str,
        struct_name: &str,
        is_key_field: bool,
        root: &IdlModule,
    ) -> Vec<String> {
        match self {
            IdlTypeSpec::ArrayType(typespec, _values) => {
                get_meta_op_complex(self, name, struct_name, is_key_field, root)
            }
            IdlTypeSpec::SequenceType(typespec, _value) => {
                get_meta_op_complex(self, name, struct_name, is_key_field, root)
            }
            IdlTypeSpec::StringType(_value) => {
                STRING.get_meta_op(name, struct_name, is_key_field, root)
            }
            IdlTypeSpec::WideStringType(_value) => vec![String::from("NOT IMPLEMENTED")],
            IdlTypeSpec::F32Type => FLOAT.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::F64Type => DOUBLE.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::F128Type => vec![String::from("NOT IMPLEMENTED")],
            IdlTypeSpec::I16Type => SHORT.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::I32Type => LONG.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::I64Type => LONGLONG.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::U16Type => USHORT.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::U32Type => ULONG.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::U64Type => ULONGLONG.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::CharType => CHAR.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::WideCharType => vec![String::from("NOT IMPLEMENTED")],
            IdlTypeSpec::BooleanType => BOOLEAN.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::OctetType => OCTET.get_meta_op(name, struct_name, is_key_field, root),
            IdlTypeSpec::ScopedName(scoped_name) => {
                
                if let Some(scoped_name_type) = root.get_type_decl(&scoped_name) {
                    let mut meta_ops = Vec::new();
                    match &scoped_name_type.0 {
                        IdlTypeDclKind::StructDcl(id, members, is_key) => {
                            for m in members {
                                let field_name = format!("{}.{}",name,m.id);
                                meta_ops.append(&mut m.type_spec.get_meta_op(&field_name,struct_name,*is_key,root))
                            }
                        }
                        _ => panic!("Only structs supported in ScopedName")
                    }
                    meta_ops
                } else {
                    panic!("Unable to determine type of scoped name:{:?}",name);
                }
            },
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_sub_op(&self, root: &IdlModule) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => STRING.get_sub_op(root),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_sub_op(root),
            IdlTypeSpec::F64Type => DOUBLE.get_sub_op(root),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_sub_op(root),
            IdlTypeSpec::I32Type => LONG.get_sub_op(root),
            IdlTypeSpec::I64Type => LONGLONG.get_sub_op(root),
            IdlTypeSpec::U16Type => USHORT.get_sub_op(root),
            IdlTypeSpec::U32Type => ULONG.get_sub_op(root),
            IdlTypeSpec::U64Type => ULONGLONG.get_sub_op(root),
            IdlTypeSpec::CharType => CHAR.get_sub_op(root),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_sub_op(root),
            IdlTypeSpec::OctetType => OCTET.get_sub_op(root),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }

    //DDS_OP_ADR | DDS_OP_TYPE_SEQ | DDS_OP_SUBTYPE_STR, offsetof (TestData_Msg, sequence_field),
    fn get_op(&self, root: &IdlModule) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_op(root),
            IdlTypeSpec::F64Type => DOUBLE.get_op(root),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_op(root),
            IdlTypeSpec::I32Type => LONG.get_op(root),
            IdlTypeSpec::I64Type => LONGLONG.get_op(root),
            IdlTypeSpec::U16Type => USHORT.get_op(root),
            IdlTypeSpec::U32Type => ULONG.get_op(root),
            IdlTypeSpec::U64Type => ULONGLONG.get_op(root),
            IdlTypeSpec::CharType => CHAR.get_op(root),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_op(root),
            IdlTypeSpec::OctetType => OCTET.get_op(root),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_c_type(&self, root: &IdlModule) -> String {
        match self {
            IdlTypeSpec::ArrayType(typespec, _values) => {
                // The array sizes are handled at the StructMember.
                typespec.get_c_type(root)
            }
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("dds_sequence_t"),
            IdlTypeSpec::StringType(_value) => STRING.get_c_type(root),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_c_type(root),
            IdlTypeSpec::F64Type => DOUBLE.get_c_type(root),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_c_type(root),
            IdlTypeSpec::I32Type => LONG.get_c_type(root),
            IdlTypeSpec::I64Type => LONGLONG.get_c_type(root),
            IdlTypeSpec::U16Type => USHORT.get_c_type(root),
            IdlTypeSpec::U32Type => ULONG.get_c_type(root),
            IdlTypeSpec::U64Type => ULONGLONG.get_c_type(root),
            IdlTypeSpec::CharType => CHAR.get_c_type(root),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_c_type(root),
            IdlTypeSpec::OctetType => OCTET.get_c_type(root),
            IdlTypeSpec::ScopedName(name) => name.get_c_type(root),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_xml(&self, root: &IdlModule) -> String {
        match self {
            IdlTypeSpec::ArrayType(_typespec, _values) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::SequenceType(_typespec, _value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::StringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::WideStringType(_value) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::F32Type => FLOAT.get_xml(root),
            IdlTypeSpec::F64Type => DOUBLE.get_xml(root),
            IdlTypeSpec::F128Type => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::I16Type => SHORT.get_xml(root),
            IdlTypeSpec::I32Type => LONG.get_xml(root),
            IdlTypeSpec::I64Type => LONGLONG.get_xml(root),
            IdlTypeSpec::U16Type => USHORT.get_xml(root),
            IdlTypeSpec::U32Type => ULONG.get_xml(root),
            IdlTypeSpec::U64Type => ULONGLONG.get_xml(root),
            IdlTypeSpec::CharType => CHAR.get_xml(root),
            IdlTypeSpec::WideCharType => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_xml(root),
            IdlTypeSpec::OctetType => OCTET.get_xml(root),
            IdlTypeSpec::ScopedName(_name) => String::from("NOT IMPLEMENTED"),
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }

    fn get_key_size(&self, root: &IdlModule) -> i32 {
        0
    }

    fn get_meta_op_size(&self, root: &IdlModule) -> i32 {
        match self {
            IdlTypeSpec::ArrayType(typespec, _values) => {
                match (*typespec).deref() {
                    IdlTypeSpec::StringType(_) => 5,
                    IdlTypeSpec::ScopedName(name) => {
                        if let Some(t) = root.get_type_decl(name) {
                            match &t.0 {
                                IdlTypeDclKind::StructDcl(_id, members, _is_key) => {
                                    let sum = members
                                        .iter()
                                        .fold(0, |sum, x| sum + x.type_spec.get_meta_op_size(root));
                                    6 + sum
                                }
                                _ => panic!("Unsupported Scoped name:{:?}", name),
                            }
                        } else {
                            panic!("Unable to find type decl for scoped name:{:?}", name);
                        }
                    }
                    _ => 1 + typespec.get_meta_op_size(root), // basic types
                }
            }
            IdlTypeSpec::SequenceType(typespec, _value) => match (*typespec).deref() {
                IdlTypeSpec::StringType(_) => 2,
                IdlTypeSpec::ScopedName(name) => {
                    if let Some(t) = root.get_type_decl(name) {
                        match &t.0 {
                            IdlTypeDclKind::StructDcl(_id, members, _is_key) => {
                                let sum = members
                                    .iter()
                                    .fold(0, |sum, x| sum + x.type_spec.get_meta_op_size(root));
                                5 + sum
                            }
                            _ => panic!("Unsupported Scoped name:{:?}", name),
                        }
                    } else {
                        panic!("Unable to find type decl for scoped name:{:?}", name);
                    }
                }
                _ => typespec.get_meta_op_size(root),
            },
            IdlTypeSpec::StringType(_value) => STRING.get_meta_op_size(root),
            IdlTypeSpec::WideStringType(_value) => 0,
            IdlTypeSpec::F32Type => FLOAT.get_meta_op_size(root),
            IdlTypeSpec::F64Type => DOUBLE.get_meta_op_size(root),
            IdlTypeSpec::F128Type => panic!("Unimplemented F128Type"),
            IdlTypeSpec::I16Type => SHORT.get_meta_op_size(root),
            IdlTypeSpec::I32Type => LONG.get_meta_op_size(root),
            IdlTypeSpec::I64Type => LONGLONG.get_meta_op_size(root),
            IdlTypeSpec::U16Type => USHORT.get_meta_op_size(root),
            IdlTypeSpec::U32Type => ULONG.get_meta_op_size(root),
            IdlTypeSpec::U64Type => ULONGLONG.get_meta_op_size(root),
            IdlTypeSpec::CharType => CHAR.get_meta_op_size(root),
            IdlTypeSpec::WideCharType => panic!("Not implemented WideChar"),
            IdlTypeSpec::BooleanType => BOOLEAN.get_meta_op_size(root),
            IdlTypeSpec::OctetType => OCTET.get_meta_op_size(root),
            IdlTypeSpec::ScopedName(name) => {
                if let Some(t) = root.get_type_decl(name) {
                    match &t.0 {
                        IdlTypeDclKind::StructDcl(id, members, is_key) => {
                            let sum = members
                                .iter()
                                .fold(0, |sum, x| sum + x.type_spec.get_meta_op_size(root));
                            sum
                        }
                        _ => panic!("Unsupported Scoped name:{:?}", name),
                    }
                } else {
                    panic!("Unable to find type decl for scoped name:{:?}", name);
                }
            }
            IdlTypeSpec::None => panic!("Unexpected get_meta_op for IdlTypeSpec::None"),
        }
    }
    fn get_alignment(&self, root: &IdlModule) -> Alignment {
        Alignment::new(AlignmentType::Ptr)
    }

    fn contains_union(&self, root: &IdlModule) -> bool {
        false
    }
}

impl IdlTypeSpec {
    pub fn write_h<W: Write>(&self, out: &mut W, root: &IdlModule) -> Result<(), Error> {
        out.write_all(self.get_c_type(root).as_bytes())?;

        Ok(())
    }
}

impl Type for IdlScopedName {
    fn get_meta_op(
        &self,
        name: &str,
        struct_name: &str,
        is_key_field: bool,
        root: &IdlModule,
    ) -> Vec<String> {
        panic!("Unimplemented");
    }
    fn get_sub_op(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_op(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_c_type(&self, root: &IdlModule) -> String {
        let is_absolute_path = self.1;
        let components = &self.0;
        if is_absolute_path {
            components.join("_")
        } else {
            String::from(components.iter().last().as_deref().unwrap())
        }
    }
    fn get_xml(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_key_size(&self, root: &IdlModule) -> i32 {
        panic!("Unimplemented");
    }
    fn get_meta_op_size(&self, root: &IdlModule) -> i32 {
        panic!("Unimplemented");
    }
    fn get_alignment(&self, root: &IdlModule) -> Alignment {
        panic!("Unimplemented");
    }
    fn contains_union(&self, root: &IdlModule) -> bool {
        panic!("Unimplemented");
    }
}


impl Type for IdlTypeDcl {
    fn get_meta_op(
        &self,
        name: &str,
        struct_name: &str,
        is_key_field: bool,
        root: &IdlModule,
    ) -> Vec<String> {

        let mut meta_ops = Vec::new();
        match &self.0 {
            IdlTypeDclKind::StructDcl(id,members,is_key) => {
                for m in members {
                    match *m.type_spec {
                        IdlTypeSpec::ScopedName(ref name) => meta_ops.append( &mut m.type_spec.get_meta_op(&m.id,&id,m.is_key,root)),
                        _ => meta_ops.append( &mut m.type_spec.get_meta_op(&m.id,&id,m.is_key,root))
                    }
                }
                meta_ops.append(&mut vec![String::from("DDS_OP_RTS")]);
            }
            _ => panic!("Unsupported IdlTypeDeclKind")
        }

        
        meta_ops
    }
    fn get_sub_op(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_op(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_c_type(&self, root: &IdlModule) -> String {
        String::from("UNIMPLEMENTED")
    }
    fn get_xml(&self, root: &IdlModule) -> String {
        panic!("Unimplemented");
    }
    fn get_key_size(&self, root: &IdlModule) -> i32 {
        panic!("Unimplemented");
    }
    fn get_meta_op_size(&self, root: &IdlModule) -> i32 {
        panic!("Unimplemented");
    }
    fn get_alignment(&self, root: &IdlModule) -> Alignment {
        panic!("Unimplemented");
    }
    fn contains_union(&self, root: &IdlModule) -> bool {
        panic!("Unimplemented");
    }
}