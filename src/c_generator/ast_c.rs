// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

/* C File generator */
use crate::{IdlModule, IdlStructMember, IdlTypeDcl, IdlTypeDclKind};

use std::io::Error;
use std::io::Write;

impl IdlModule {
    // Write C header file
    pub fn write_c<W: Write>(&mut self, out: &mut W, scope: &Vec<String>) -> Result<(), Error> {
        let mut scope = scope.clone();

        if let Some(id) = self.id.as_ref() {
            scope.push(id.clone());
        }

        writeln!(out, "")?;

        for typ in self.types.entries() {
            typ.into_mut().write_c(out, &scope)?;
        }

        writeln!(out, "")?;
        for module in self.modules.entries() {
            module.into_mut().write_c(out, &scope)?;
        }

        Ok(())
    }
}

impl IdlTypeDcl {
    pub fn write_c<W: Write>(&mut self, out: &mut W, scope: &Vec<String>) -> Result<(), Error> {
        match self.0 {
            IdlTypeDclKind::StructDcl(ref id, ref members, is_key) => {
                if is_key {}

                Ok(())
            }
            _ => panic!("Unsupported {:?}", self),
        }
    }
}
