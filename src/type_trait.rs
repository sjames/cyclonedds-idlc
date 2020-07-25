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
use crate::alignment::Alignment;

pub trait Type {
    fn get_meta_op(name: &str, struct_name: &str) -> Vec<String>;
    fn get_sub_op() -> String;
    fn get_op() -> String;
    fn get_c_type() -> String;
    fn get_xml() -> String;
    fn make_key_field();
    fn is_key_field() -> bool;
    fn get_key_size() -> u32;
    fn get_meta_op_size() -> u32;
    fn get_alignment() -> Alignment;
    fn contains_union() -> bool;
}
