// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

/*
public interface Type
{
  public ArrayList <String> getMetaOp (String myname, String structname);
  public String getSubOp ();
  public String getOp ();
  public String getCType ();
  public void getXML (StringBuffer str, ModuleContext mod);
  public void populateDeps (Set <ScopedName> depset, NamedType current);
  public boolean depsOK (Set <ScopedName> deps);
  public void makeKeyField ();
  public boolean isKeyField ();
  public long getKeySize ();
  public int getMetaOpSize ();
  public Alignment getAlignment ();
  public Type dup ();
  public boolean containsUnion ();
}
*/
use crate::c_generator::alignment::Alignment;
use crate::IdlModule;

pub trait Type {
    fn get_meta_op(
        &self,
        name: &str,
        struct_name: &str,
        is_key_field: bool,
        root: &IdlModule,
    ) -> String;
    fn get_sub_op(&self, root: &IdlModule) -> String;
    fn get_op(&self, root: &IdlModule) -> String;
    fn get_c_type(&self, root: &IdlModule) -> String;
    fn get_xml(&self, root: &IdlModule) -> String;
    fn get_key_size(&self, root: &IdlModule) -> i32;
    fn get_meta_op_size(&self, root: &IdlModule) -> i32;
    fn get_alignment(&self, root: &IdlModule) -> Alignment;
    fn contains_union(&self, root: &IdlModule) -> bool;
}
