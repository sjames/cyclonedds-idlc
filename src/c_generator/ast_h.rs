use crate::{IdlModule, IdlStructMember, IdlTypeDcl, IdlTypeDclKind};

use std::io::Error;
use std::io::Write;

const INDENTION: usize = 4;

impl IdlModule {
    // Write C header file
    pub fn write_h<W: Write>(&mut self, out: &mut W, scope: &Vec<String>) -> Result<(), Error> {
        let mut scope = scope.clone();
        if let Some(id) = self.id.as_ref() {
            scope.push(id.clone());
        }

        for typ in self.types.entries() {
            typ.into_mut().write_h(out, &scope)?;
        }

        for module in self.modules.entries() {
            module.into_mut().write_h(out, &scope)?;
        }

        for cnst in self.constants.entries() {
            cnst.into_mut().write(out, 0)?;
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

fn scoped_name(scope: &Vec<String>, id: &str) -> String {
    let mut name = scope.join("_");
    name.push_str("_");
    name.push_str(id);
    String::from(name)
}

impl IdlTypeDcl {
    pub fn write_h<W: Write>(&mut self, out: &mut W, scope: &Vec<String>) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::StructDcl(ref id, ref members) => {
                //typedef struct HelloWorldData_Msg
                let _ = writeln!(out, "typedef struct {}", &scoped_name(scope, id));
                let _ = writeln!(out, "{{");

                for member in members {
                    let _ = write!(out, "{:indent$} ", "", indent = (0 + 1) * INDENTION)
                        .and_then(|_| member.as_ref().write_h(out, 0 + 1))
                        .and_then(|_| writeln!(out));
                }

                let _ = writeln!(out, "}} {};", &scoped_name(scope, id));

                Ok(())
            }
            _ => panic!("Unsupported {:?}", self),
        }
    }
}

impl IdlStructMember {
    ///
    pub fn write_h<W: Write>(&self, out: &mut W, _level: usize) -> Result<(), Error> {
        self.type_spec.write_h(out)?;
        write!(out, " {};", self.id)
    }
}
