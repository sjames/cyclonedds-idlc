// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

use crate::{IdlModule, IdlStructMember, IdlTypeDcl, IdlTypeDclKind, IdlTypeSpec};

use std::io::Error;
use std::io::Write;

use crate::c_generator::type_trait::Type;
use crate::c_generator::{scoped_name, INDENTION};

impl IdlModule {
    // Write C header file
    pub fn write_h<W: Write>(
        &self,
        out: &mut W,
        scope: &Vec<String>,
        root: &IdlModule,
    ) -> Result<(), Error> {
        let mut scope = scope.clone();
        if let Some(id) = self.id.as_ref() {
            scope.push(id.clone());
        }
        writeln!(out, "")?;
        for (_, typ) in self.types.iter() {
            typ.write_h(out, &scope, root)?;
        }

        writeln!(out, "")?;
        for (_, module) in self.modules.iter() {
            module.write_h(out, &scope, root)?;
        }

        writeln!(out, "")?;
        for (_, cnst) in self.constants.iter() {
            cnst.write(out, 0, root)?;
        }

        writeln!(out, "")?;

        for (_, typ) in self.types.iter() {
            typ.write_desc_declaration(out, &scope)?;
        }

        writeln!(out, "")?;

        for (_, typ) in self.types.iter() {
            typ.write_allocator_macro(out, &scope)?;
        }

        Ok(())
    }
}

/*
    None,
    TypeDcl(String, Box<IdlTypeSpec>),
    StructDcl(String, Vec<Box<IdlStructMember>>),
    UnionDcl(String, Box<IdlTypeSpec>, Vec<IdlSwitchCase>),
    EnumDcl(String, Vec<String>),
*/

impl IdlTypeDcl {
    pub fn write_h<W: Write>(
        &self,
        out: &mut W,
        scope: &Vec<String>,
        root: &IdlModule,
    ) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::StructDcl(ref id, ref members, _) => {
                //typedef struct HelloWorldData_Msg
                let _ = writeln!(out, "typedef struct {}", &scoped_name(scope, id));
                let _ = writeln!(out, "{{");

                for member in members {
                    let _ = write!(out, "{:indent$} ", "", indent = (0 + 1) * INDENTION)
                        .and_then(|_| member.as_ref().write_h(out, 0 + 1, scope, root))
                        .and_then(|_| writeln!(out));
                }

                let _ = writeln!(out, "}} {};", &scoped_name(scope, id));

                Ok(())
            }
            _ => panic!("Unsupported {:?}", self),
        }
    }

    pub fn write_desc_declaration<W: Write>(
        &self,
        out: &mut W,
        scope: &Vec<String>,
    ) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::StructDcl(ref id, _, is_key) => {
                //extern const dds_topic_descriptor_t HelloWorldData_Msg_desc;
                if is_key {
                    let _ = writeln!(
                        out,
                        "extern const dds_topic_descriptor_t {}_desc;",
                        &scoped_name(scope, id)
                    )?;
                }
                Ok(())
            }
            _ => panic!("Unsupported {:?}", self),
        }
    }

    pub fn write_allocator_macro<W: Write>(
        &self,
        out: &mut W,
        scope: &Vec<String>,
    ) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::StructDcl(ref id, _, is_key) => {
                if is_key {
                    let alloc_str = std::include_str!("templates/allocator_macro.txt");
                    let _ = out.write(
                        alloc_str
                            .replace("<SCOPED_NAME>", &scoped_name(scope, id))
                            .as_bytes(),
                    )?;
                }
                Ok(())
            }
            _ => panic!("Unsupported {:?}", self),
        }
    }
}

impl IdlStructMember {
    ///
    pub fn write_h<W: Write>(
        &self,
        out: &mut W,
        level: usize,
        scope: &Vec<String>,
        root: &IdlModule,
    ) -> Result<(), Error> {
        match self.type_spec.as_ref() {
            IdlTypeSpec::ArrayType(spec, values) => {
                // Array types in c are different. The array size comes after the id
                self.type_spec.write_h(out, root)?;
                write!(out, " {}", self.id)?;
                for value in values {
                    write!(out, "[")?;
                    value.write(out)?;
                    write!(out, "]")?;
                }
                write!(out, ";")
            }
            IdlTypeSpec::ScopedName(name) => {
                let is_absolute_path = name.1;
                if !is_absolute_path {
                    out.write_all(&scope.join("_").as_bytes())?;
                    write!(out, "_")?;
                }
                self.type_spec.write_h(out, root)?;
                write!(out, " {};", self.id)
            }
            _ => {
                self.type_spec.write_h(out, root)?;
                write!(out, " {};", self.id)
            }
        }
    }
}
