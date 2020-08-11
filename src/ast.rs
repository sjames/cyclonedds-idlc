// Copyright (C) 2019  Frank Rehberger
// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>
use crate::IdlError;
use linked_hash_map::LinkedHashMap;
use std::io::Error;
use std::io::Write;

use crate::c_generator::type_trait::Type;

///
#[derive(Clone, Debug)]
pub enum UnaryOp {
    Neg,
    Pos,
    Inverse,
}

const INDENTION: usize = 4;
const ATTR_ALLOW_DEADCODE: &str = "#[allow(dead_code)]";
const ATTR_DERIVE_SERDE: &str = "#[derive(Serialize, Deserialize)]";
const ATTR_DERIVE_CLONE_DEBUG: &str = "#[derive(Clone, Debug)]";
const ATTR_ALLOW_NON_CAMEL_CASE_TYPES: &str = "#[allow(non_camel_case_types)]";
const ATTR_ALLOW_NON_SNAKE_CASE: &str = "#[allow(non_snake_case)]";
const IMPORT_SERDE: &str = "use serde_derive::{Serialize, Deserialize};";
const ATTR_ALLOW_UNUSED_IMPORTS: &str = "#[allow(unused_imports)]";
const ATTR_REPL_C: &str = "#[repr(C)]";

impl UnaryOp {
    pub fn write<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let _ = match self {
            UnaryOp::Neg => write!(out, "-"),
            UnaryOp::Pos => write!(out, "+"),
            UnaryOp::Inverse => write!(out, "~"),
        };
        Ok(())
    }
}

///
#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LShift,
    RShift,
    Or,
    Xor,
    And,
}

impl BinaryOp {
    pub fn write<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let _ = match self {
            BinaryOp::Add => write!(out, "+"),
            BinaryOp::Sub => write!(out, "-"),
            BinaryOp::Mul => write!(out, "*"),
            BinaryOp::Div => write!(out, "/"),
            BinaryOp::Mod => write!(out, "%"),
            BinaryOp::LShift => write!(out, "<<"),
            BinaryOp::RShift => write!(out, ">>"),
            BinaryOp::Or => write!(out, "|"),
            BinaryOp::Xor => write!(out, "^"),
            BinaryOp::And => write!(out, "&"),
        };
        Ok(())
    }
}

///
#[derive(Clone, Debug)]
pub struct IdlScopedName(pub Vec<String>, pub bool);

impl IdlScopedName {
    pub fn write<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let is_absolute_path = self.1;
        let components = &self.0;
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
        Ok(())
    }
}

///
#[derive(Clone, Debug)]
pub enum IdlValueExpr {
    None,
    DecLiteral(String),
    HexLiteral(String),
    OctLiteral(String),
    CharLiteral(String),
    WideCharLiteral(String),
    StringLiteral(String),
    WideStringLiteral(String),
    BooleanLiteral(bool),
    FloatLiteral(
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
    UnaryOp(UnaryOp, Box<IdlValueExpr>),
    BinaryOp(BinaryOp, Box<IdlValueExpr>),
    Expr(Box<IdlValueExpr>, Box<IdlValueExpr>),
    Brace(Box<IdlValueExpr>),
    ScopedName(IdlScopedName),
}

impl IdlValueExpr {
    pub fn write<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let _ = match self {
            IdlValueExpr::None => write!(out, ""),
            IdlValueExpr::DecLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::HexLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::OctLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::CharLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::WideCharLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::StringLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::WideStringLiteral(ref val) => write!(out, "{}", val),
            IdlValueExpr::BooleanLiteral(val) => write!(out, "{}", val),
            //            FloatLiteral(ref integ => write!(out, "{}", val), ref fract, ref expo, ref suffix) => write!(out, "{}", val),
            IdlValueExpr::UnaryOp(op, ref expr) => op.write(out).and_then(|_| expr.write(out)),
            IdlValueExpr::BinaryOp(op, ref expr) => op.write(out).and_then(|_| expr.write(out)),
            IdlValueExpr::Expr(ref expr1, ref expr2) => {
                expr1.write(out).and_then(|_| expr2.write(out))
            }
            IdlValueExpr::Brace(ref expr) => write!(out, "{}", "(")
                .and_then(|_| expr.write(out))
                .and_then(|_| write!(out, "{}", ")")),
            IdlValueExpr::FloatLiteral(ref integral, ref fraction, ref exponent, ref suffix) => {
                integral.as_ref().and_then(|i| write!(out, "{}", i).err());
                fraction.as_ref().and_then(|f| write!(out, ".{}", f).err());
                exponent.as_ref().and_then(|e| write!(out, "e{}", e).err());
                suffix.as_ref().and_then(|s| write!(out, "{}", s).err());
                Ok(())
            }
            IdlValueExpr::ScopedName(ref name) => name.write(out),
            //_ => unimplemented!(),
        };
        Ok(())
    }
}

///
impl Default for IdlValueExpr {
    fn default() -> IdlValueExpr {
        IdlValueExpr::None
    }
}

///
#[derive(Clone, Debug)]
pub struct IdlStructMember {
    pub id: String,
    pub type_spec: Box<IdlTypeSpec>,
    pub is_key: bool,
}

///
impl IdlStructMember {
    ///
    pub fn write<W: Write>(&self, out: &mut W, _level: usize) -> Result<(), Error> {
        write!(out, "{}: ", self.id)
            .and_then(|_| self.type_spec.write(out))
            .and_then(|_| write!(out, ","))
    }
}

///
#[derive(Clone, Debug)]
pub struct IdlSwitchElement {
    pub id: String,
    pub type_spec: Box<IdlTypeSpec>,
}

///
impl IdlSwitchElement {
    ///
    pub fn write<W: Write>(&self, out: &mut W, _level: usize) -> Result<(), Error> {
        write!(out, "{}: ", self.id)
            .and_then(|_| self.type_spec.write(out))
            .and_then(|_| write!(out, ","))
    }
}

///
#[derive(Clone, Debug)]
pub enum IdlSwitchLabel {
    Label(Box<IdlValueExpr>),
    Default,
}

///
#[derive(Clone, Debug)]
pub struct IdlSwitchCase {
    pub labels: Vec<IdlSwitchLabel>,
    pub elem_spec: Box<IdlSwitchElement>,
}

///
impl IdlSwitchCase {
    ///
    pub fn write<W: Write>(&self, out: &mut W, level: usize) -> Result<(), Error> {
        for label in &self.labels {
            match label {
                IdlSwitchLabel::Label(ref val_expr) => {
                    write!(out, "{:indent$}", "", indent = level * INDENTION)
                        .and_then(|_| val_expr.write(out))
                        .and_then(|_| write!(out, "{}", "{"))
                        .and_then(|_| self.elem_spec.write(out, level + 1))
                        .and_then(|_| writeln!(out, "{}", "},"))?
                }
                IdlSwitchLabel::Default => write!(
                    out,
                    "{:indent$}default{}",
                    "",
                    "{",
                    indent = level * INDENTION
                )
                .and_then(|_| self.elem_spec.write(out, level + 1))
                .and_then(|_| writeln!(out, "{}", "},"))?,
            }
        }
        Ok(())
    }
}

///
#[derive(Clone, Debug)]
pub enum IdlTypeSpec {
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
}

///
impl IdlTypeSpec {
    ///
    pub fn write<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let _ = match self {
            IdlTypeSpec::F32Type => write!(out, "f32"),
            IdlTypeSpec::F64Type => write!(out, "f64"),
            IdlTypeSpec::F128Type => write!(out, "f128"),
            IdlTypeSpec::I16Type => write!(out, "i16"),
            IdlTypeSpec::I32Type => write!(out, "i32"),
            IdlTypeSpec::I64Type => write!(out, "i64"),
            IdlTypeSpec::U16Type => write!(out, "u16"),
            IdlTypeSpec::U32Type => write!(out, "u32"),
            IdlTypeSpec::U64Type => write!(out, "u64"),
            IdlTypeSpec::CharType => write!(out, "char"),
            IdlTypeSpec::WideCharType => write!(out, "char"),
            IdlTypeSpec::BooleanType => write!(out, "bool"),
            IdlTypeSpec::OctetType => write!(out, "u8"),
            IdlTypeSpec::StringType(None) => write!(out, "*mut ::std::os::raw::c_char"),
            IdlTypeSpec::WideStringType(None) => write!(out, "*mut ::std::os::raw::c_char"),
            // TODO implement String/Sequence bounds
            IdlTypeSpec::StringType(_) => write!(out, "String"),
            // TODO implement String/Sequence bounds for serializer and deserialzer
            IdlTypeSpec::WideStringType(_) => write!(out, "String"),
            IdlTypeSpec::SequenceType(_, _) => write!(out, "dds_sequence_t"),
            /*
            IdlTypeSpec::SequenceType(typ_expr, _) => write!(out, "Vec<")
                .and_then(|_| typ_expr.as_ref().write(out))
                .and_then(|_| write!(out, ">")),*/
            IdlTypeSpec::ArrayType(typ_expr, dim_expr_list) => {
                for _ in dim_expr_list {
                    let _ = write!(out, "[");
                }
                let _ = typ_expr.as_ref().write(out);
                for dim_expr in dim_expr_list {
                    // TODO return result
                    let _ = write!(out, ";")
                        .and_then(|_| dim_expr.as_ref().write(out))
                        .and_then(|_| write!(out, "]"));
                }
                Ok(())
            }
            IdlTypeSpec::ScopedName(ref name) => name.write(out),
            _ => unimplemented!(),
        };

        Ok(())
    }
}

///
impl Default for IdlTypeSpec {
    fn default() -> IdlTypeSpec {
        IdlTypeSpec::None
    }
}

///
#[derive(Clone, Debug)]
pub enum IdlTypeDclKind {
    None,
    TypeDcl(String, Box<IdlTypeSpec>),
    StructDcl(String, Vec<Box<IdlStructMember>>, bool),
    UnionDcl(String, Box<IdlTypeSpec>, Vec<IdlSwitchCase>),
    EnumDcl(String, Vec<String>),
}

///
impl Default for IdlTypeDclKind {
    fn default() -> IdlTypeDclKind {
        IdlTypeDclKind::None
    }
}

///
#[derive(Clone, Debug, Default)]
pub struct IdlTypeDcl(pub IdlTypeDclKind);

///
impl IdlTypeDcl {
    ///
    ///
    pub fn write<W: Write>(
        &self,
        out: &mut W,
        level: usize,
        root: &IdlModule,
    ) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::TypeDcl(ref id, ref type_spec) => {
                // TODO collect/return result
                let _ = writeln!(out, "");
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_DEADCODE,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_NON_CAMEL_CASE_TYPES,
                    indent = level * INDENTION
                );
                let _ = write!(
                    out,
                    "{:indent$}pub type {} = ",
                    "",
                    id,
                    indent = level * INDENTION
                );
                let _ = type_spec.as_ref().write(out);
                let _ = writeln!(out, ";");
                Ok(())
            }
            IdlTypeDclKind::StructDcl(ref id, ref type_spec, is_key) => {
                // TODO collect/return result
                let _ = writeln!(out, "");
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_DEADCODE,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_NON_CAMEL_CASE_TYPES,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_REPL_C,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_DERIVE_CLONE_DEBUG,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}pub struct {} {}",
                    "",
                    id,
                    "{",
                    indent = level * INDENTION
                );
                for member in type_spec {
                    let _ = write!(out, "{:indent$}pub ", "", indent = (level + 1) * INDENTION)
                        .and_then(|_| member.as_ref().write(out, level + 1))
                        .and_then(|_| writeln!(out));
                }

                let _ = writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION);

                // Implementation needed only for key structures
                if is_key {
                    let _ = writeln!(
                        out,
                        "{:indent$}pub struct {}_desc {}",
                        "",
                        id,
                        "{",
                        indent = level * INDENTION
                    );

                    let num_keys = type_spec.iter().filter(|m| m.is_key).count();
                    let num_ops = type_spec
                        .iter()
                        .fold(0, |acc, x| acc + x.type_spec.get_meta_op_size(root))
                        + 1;

                    let _ = writeln!(
                        out,
                        "{:indent$}key_descriptor : [dds_key_descriptor;{}],",
                        "",
                        num_keys,
                        indent = (level + 1) * INDENTION
                    );
                    let _ = writeln!(
                        out,
                        "{:indent$}ops : [u32;{}],",
                        "",
                        num_ops,
                        indent = (level + 1) * INDENTION
                    );
                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "descriptor : dds_topic_descriptor,",
                        indent = (level + 1) * INDENTION
                    );
                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "_pin: PhantomPinned,",
                        indent = (level + 1) * INDENTION
                    );
                    let _ = writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION);

                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "//Implementation",
                        indent = level * INDENTION
                    );

                    let _ = writeln!(
                        out,
                        "{:indent$}impl {} {}",
                        "",
                        id,
                        "{",
                        indent = level * INDENTION
                    );
                    let _ = writeln!(
                        out,
                        "{:indent$}pub fn descriptor() -> Pin<Box<{}_desc>> {}",
                        "",
                        id,
                        "{",
                        indent = (level + 1) * INDENTION
                    );

                    let _ = writeln!(
                        out,
                        "{:indent$}let mut res = {}_desc {{",
                        "",
                        id,
                        indent = (level + 2) * INDENTION
                    );

                    let _ = writeln!(
                        out,
                        "{:indent$}key_descriptor : [",
                        "",
                        indent = (level + 3) * INDENTION
                    );
                    let mut offset = 0;
                    for k in type_spec.iter() {
                        if k.is_key {
                            let _ = writeln!(
                                out,
                                "{:indent$}dds_key_descriptor {{",
                                "",
                                indent = (level + 4) * INDENTION
                            );
                            let _ = writeln!(out, "{:indent$}m_name : unsafe {{ std::ffi::CStr::from_bytes_with_nul_unchecked(b\"{}\\0\").as_ptr()}},", "", k.id, indent = (level+5) * INDENTION);
                            let _ = writeln!(
                                out,
                                "{:indent$}m_index : {},",
                                "",
                                offset,
                                indent = (level + 5) * INDENTION
                            );
                            let _ = writeln!(
                                out,
                                "{:indent$}}},",
                                "",
                                indent = (level + 4) * INDENTION
                            );
                        }
                        offset = offset + k.type_spec.get_meta_op_size(root);
                    }
                    //end of key_descriptor
                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "],",
                        indent = (level + 3) * INDENTION
                    );

                    //Begin ops
                    let _ = writeln!(
                        out,
                        "{:indent$}ops : [",
                        "",
                        indent = (level + 3) * INDENTION
                    );

                    for op in self.get_meta_op("","",false,root) {
                        let _ = writeln!(
                            out,
                            "{:indent$}{},",
                            "",
                            op,
                            indent = (level + 5) * INDENTION
                        );
                    }

                    /*
                    for m in type_spec.iter() {
                        let _ = writeln!(
                            out,
                            "{:indent$}{},",
                            "",
                            m.type_spec.get_meta_op(&m.id, &id, m.is_key, root)[0],
                            indent = (level + 5) * INDENTION
                        );
                    }
                    */
                
                    
                    //end ops
                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "],",
                        indent = (level + 3) * INDENTION
                    );

                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "};",
                        indent = (level + 2) * INDENTION
                    );

                    // end of descriptor()
                    let _ = writeln!(
                        out,
                        "{:indent$}{}",
                        "",
                        "}",
                        indent = (level + 1) * INDENTION
                    );

                    let _ = writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION);
                }

                Ok(())
            }

            IdlTypeDclKind::EnumDcl(ref id, ref enums) => {
                // TODO collect/return result
                let _ = writeln!(out, "");
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_DEADCODE,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_NON_CAMEL_CASE_TYPES,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_REPL_C,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_DERIVE_CLONE_DEBUG,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}pub enum {} {}",
                    "",
                    id,
                    "{",
                    indent = level * INDENTION
                );
                for variant in enums {
                    let _ = writeln!(
                        out,
                        "{:indent$}{},",
                        "",
                        variant,
                        indent = (level + 1) * INDENTION
                    );
                }
                let _ = writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION);
                Ok(())
            }

            IdlTypeDclKind::UnionDcl(ref id, ref _type_spec, ref switch_cases) => {
                // TODO collect/return result
                let _ = writeln!(out, "");
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_DEADCODE,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_ALLOW_NON_CAMEL_CASE_TYPES,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_REPL_C,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}{}",
                    "",
                    ATTR_DERIVE_CLONE_DEBUG,
                    indent = level * INDENTION
                );
                let _ = writeln!(
                    out,
                    "{:indent$}pub enum {} {}",
                    "",
                    id,
                    "{",
                    indent = level * INDENTION
                );
                for case in switch_cases {
                    let _ = case.write(out, level + 1);
                }
                let _ = writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION);

                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);
                let _ = writeln!(
                    out,
                    "{:indent$}// TODO custom de-/serializer",
                    "",
                    indent = level * INDENTION
                );
                let _ = writeln!(out, "{:indent$}//", "", indent = level * INDENTION);

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn get_num_ops() -> usize {
        5
    }
}

///
#[derive(Clone, Default, Debug)]
pub struct IdlConstDcl {
    pub id: String,
    pub typedcl: Box<IdlTypeSpec>,
    pub value: Box<IdlValueExpr>,
}

///
impl IdlConstDcl {
    ///
    ///
    pub fn write<W: Write>(
        &self,
        out: &mut W,
        level: usize,
        root: &IdlModule,
    ) -> Result<(), Error> {
        writeln!(
            out,
            "{:indent$}{}",
            "",
            ATTR_ALLOW_DEADCODE,
            indent = level * INDENTION
        )
        .and_then(|_| {
            write!(
                out,
                "{:indent$}pub const {}",
                "",
                self.id,
                indent = level * INDENTION
            )
        })
        .and_then(|_| write!(out, ": "))
        .and_then(|_| self.typedcl.write(out))
        .and_then(|_| write!(out, " = "))
        .and_then(|_| self.value.write(out))
        .and_then(|_| writeln!(out, ";"))
    }
}

///
#[derive(Clone, Default, Debug)]
pub struct IdlModule {
    pub id: Option<String>,
    pub level: usize,
    pub modules: LinkedHashMap<String, Box<IdlModule>>,
    pub types: LinkedHashMap<String, Box<IdlTypeDcl>>,
    pub constants: LinkedHashMap<String, Box<IdlConstDcl>>,
    // vector of (type_name, Vec<Keys>)
    pub keys: Vec<(String, Vec<String>)>,
}

///
impl IdlModule {
    pub fn new(id: Option<String>, level: usize) -> IdlModule {
        IdlModule {
            id: id,
            level: level,
            modules: LinkedHashMap::default(),
            types: LinkedHashMap::default(),
            constants: LinkedHashMap::default(),
            keys: Vec::new(),
        }
    }

    fn get_type_decl_rec(&self, scoped_name: &[String]) -> Option<&Box<IdlTypeDcl>> {
        if scoped_name.len() == 1 {
            return self
                .types
                .iter()
                .find_map(|(k, v)| if *k == scoped_name[0] { Some(v) } else { None });
        } else if scoped_name.len() > 1 {
            let modname = &scoped_name[0];
            let module = self.modules.iter().find_map(|(k, v)| {
                if k == modname {
                    v.get_type_decl_rec(&scoped_name[1..])
                } else {
                    println!("Not found module:{} cmp with {}", modname, k);
                    None
                }
            });
            module
        } else {
            println!("Not found:{:?}", scoped_name);

            None
        }
    }

    pub fn get_type_decl(&self, scoped_name: &IdlScopedName) -> Option<&Box<IdlTypeDcl>> {
        // absolute path
        self.get_type_decl_rec(&scoped_name.0)
    }

    pub fn set_topic_and_key_flags(
        &mut self,
        struct_name: &str,
        keys: &Vec<String>,
    ) -> Result<(), IdlError> {
        let mut struct_found = false;
        let mut keys_found = 0;
        for (_name, decl) in &mut self.types {
            //println!("Name:{}",name);
            match decl.0 {
                IdlTypeDclKind::StructDcl(ref name, ref mut members, ref mut is_key) => {
                    if name == struct_name {
                        *is_key = true;
                        struct_found = true;
                        //println!("{} is a topic",&struct_name);
                        for key in keys {
                            for ref mut member in members.iter_mut() {
                                //println!("Member:{}",&member.id);
                                if key == &member.id {
                                    member.is_key = true;
                                    keys_found = keys_found + 1;
                                    //println!("{} is a key",&member.id);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if struct_found && (keys_found == keys.len()) {
            Ok(())
        } else {
            println!("Error! : Keys not found: {}", String::from(keys.join(",")));
            Err(IdlError::KeyNotFound(String::from(keys.join(","))))
        }
    }

    pub fn write<W: Write>(
        &self,
        out: &mut W,
        level: usize,
        root: &IdlModule,
    ) -> Result<(), Error> {
        let _prolog = match self.id {
            Some(ref id_str) => writeln!(
                out,
                "{:indent$}{}",
                "",
                ATTR_ALLOW_NON_SNAKE_CASE,
                indent = level * INDENTION
            )
            .and_then(|_| {
                writeln!(
                    out,
                    "{:indent$}pub mod {} {}",
                    "",
                    id_str,
                    "{",
                    indent = level * INDENTION
                )
            }),

            _ => write!(out, ""),
        };

        let add: usize = if self.id.is_some() { 1 } else { 0 };

        let _ = writeln!(
            out,
            "{:indent$}{}",
            "",
            ATTR_ALLOW_UNUSED_IMPORTS,
            indent = (level + add) * INDENTION
        );
        //.and_then(|_| writeln!(out, "{:indent$}{}", "",
        //                       IMPORT_SERDE, indent = (level + add) * INDENTION));

        for (_, typ) in self.types.iter() {
            typ.write(out, level + add, root)?;
        }

        for (_, module) in self.modules.iter() {
            module.write(out, level + add, root)?;
        }

        for (_, cnst) in self.constants.iter() {
            cnst.write(out, level + add, root)?;
        }

        let _epilog = match self.id {
            Some(_) => writeln!(out, "{:indent$}{}", "", "}", indent = level * INDENTION),
            _ => write!(out, ""),
        };

        Ok(())
    }
}
