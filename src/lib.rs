// Copyright (C) 2019  Frank Rehberger
// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>
extern crate linked_hash_map;

mod ast;
mod cdds;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use rtps_idl_grammar::{IdlParser, Rule};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use std::include;
use std::path::PathBuf;

use crate::ast::*;

const MODULE_PRELUDE: &[u8] = b"#[allow(unused_imports)]
";

///
#[derive(Debug)]
pub enum IdlError {
    InternalError,
    UnexpectedItem(Rule),
    ExpectedItem(Rule),
    ErrorMesg(String),
    FileNotFound(String),
    KeyNotFound(String),
}

///
pub trait IdlLoader {
    fn load(&self, filename: &str) -> Result<String, Error>;
}

///
#[derive(Debug)]
pub struct Configuration {
    pub definition: HashMap<String, String>,
    pub verbose: bool,
    pub generate_c: bool,
    pub idl_name: PathBuf,
    pub generate_descriptors : bool, // generate the descriptors in Rust
}

///
impl Configuration {
    pub fn new(
        defs: HashMap<String, String>,
        verbose: bool,
        gen_c: bool,
        idl_name: PathBuf,
        generate_descriptors : bool,
    ) -> Configuration {
        Configuration {
            definition: defs,
            verbose: verbose,
            generate_c: gen_c,
            idl_name,
            generate_descriptors,
        }
    }
}

///
impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            definition: HashMap::default(),
            verbose: false,
            generate_c: false,
            idl_name: PathBuf::default(),
            generate_descriptors : false,
        }
    }
}

///
pub fn load_idl(_config: &Configuration, _path: &str) -> Result<String, Error> {
    return Ok("".to_owned());
}

///
type Scope = Vec<String>;

///
#[derive(Debug, Clone)]
struct Context<'i> {
    config: &'i Configuration,
    definitions: HashMap<String, String>,
    root_module: Box<IdlModule>,
}

impl<'i> Context<'i> {
    pub fn new(config: &'i Configuration) -> Context {
        Context {
            config: &config,
            definitions: HashMap::default(),
            root_module: Box::new(IdlModule::new(None, 0)),
        }
    }

    ///
    fn lookup_module(&mut self, scope: &Scope) -> &mut Box<IdlModule> {
        // Starting from Root traverse the scope-path
        let mut current_module = &mut self.root_module;
        let level = scope.len();
        let mut submodule_scope = Vec::new();
        //println!("Lookup module: scope:{:?}",&scope);
        for name in scope {
            submodule_scope.push(name.clone());
            
            let submodule = current_module
                .modules
                .entry(name.to_owned())
                .or_insert(Box::new(IdlModule::new(Some(name.to_owned()),level)));
            current_module = submodule;
        }

        return current_module;
    }

    ///
    fn add_type_dcl(
        &mut self,
        scope: &mut Scope,
        key: String,
        type_dcl: Box<IdlTypeDcl>,
    ) -> Result<(), IdlError> {
        let current_module = self.lookup_module(scope);
        current_module.types.entry(key).or_insert(type_dcl);

        Ok(())
    }

    ///
    fn add_const_dcl(
        &mut self,
        scope: &mut Scope,
        key: String,
        const_dcl: Box<IdlConstDcl>,
    ) -> Result<(), IdlError> {
        let current_module = self.lookup_module(scope);
        current_module.constants.entry(key).or_insert(const_dcl);

        Ok(())
    }

    fn add_other_directive(&mut self, scope: &mut Scope, directive: &str) -> Result<(), IdlError> {
        let mut directive = directive.trim_end().split(" ");

        if let (Some("#pragma"), Some("keylist"), Some(struct_name)) =
            (directive.next(), directive.next(), directive.next())
        {
            let mut keys: Vec<String> = Vec::new();
            while let Some(key) = directive.next() {
                //println!("Key: {}", key);
                keys.push(key.into());
            }
            let current_module = self.lookup_module(scope);
            current_module.set_topic_and_key_flags(struct_name, &keys)?;
        } else {
            //println!("Ignoring unknown directive");
        }

        Ok(())
    }

    // type_spec = { template_type_spec | simple_type_spec }
    pub fn read_type_spec(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Box<IdlTypeSpec>, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        match pair.as_rule() {
            Rule::float => Ok(Box::new(IdlTypeSpec::F32Type)),
            Rule::double => Ok(Box::new(IdlTypeSpec::F64Type)),
            Rule::long_double => Ok(Box::new(IdlTypeSpec::F128Type)),
            Rule::unsigned_short_int => Ok(Box::new(IdlTypeSpec::U16Type)),
            Rule::unsigned_longlong_int => Ok(Box::new(IdlTypeSpec::U64Type)),
            Rule::unsigned_long_int => Ok(Box::new(IdlTypeSpec::U32Type)),
            Rule::signed_short_int => Ok(Box::new(IdlTypeSpec::I16Type)),
            Rule::signed_longlong_int => Ok(Box::new(IdlTypeSpec::I64Type)),
            Rule::signed_long_int => Ok(Box::new(IdlTypeSpec::I32Type)),
            Rule::char_type => Ok(Box::new(IdlTypeSpec::CharType)),
            Rule::wide_char_type => Ok(Box::new(IdlTypeSpec::WideCharType)),
            Rule::boolean_type => Ok(Box::new(IdlTypeSpec::BooleanType)),
            Rule::octet_type => Ok(Box::new(IdlTypeSpec::OctetType)),
            Rule::string_type => match iter.next() {
                None => Ok(Box::new(IdlTypeSpec::StringType(None))),
                Some(ref p) => {
                    let pos_int_const = self.read_const_expr(scope, p)?;
                    Ok(Box::new(IdlTypeSpec::StringType(Some(pos_int_const))))
                }
            },
            Rule::wide_string_type => match iter.next() {
                None => Ok(Box::new(IdlTypeSpec::WideStringType(None))),
                Some(ref p) => {
                    let pos_int_const = self.read_const_expr(scope, p)?;
                    Ok(Box::new(IdlTypeSpec::WideStringType(Some(pos_int_const))))
                }
            },
            Rule::sequence_type => match (iter.next(), iter.next()) {
                (Some(ref typ), None) => {
                    let typ_expr = self.read_type_spec(scope, typ)?;
                    Ok(Box::new(IdlTypeSpec::SequenceType(typ_expr, None)))
                }
                (Some(ref typ), Some(ref bound)) => {
                    let typ_expr = self.read_type_spec(scope, typ)?;
                    let bound_expr = self.read_const_expr(scope, bound)?;
                    Ok(Box::new(IdlTypeSpec::SequenceType(
                        typ_expr,
                        Some(bound_expr),
                    )))
                }
                _ => panic!(),
            },

            //  scoped_name = { "::"? ~ identifier ~ ("::" ~ identifier)* }
            Rule::scoped_name => {
                let name = self.read_scoped_name(scope, pair)?;
                Ok(Box::new(IdlTypeSpec::ScopedName(name)))
            }

            // go deeper
            _ => {
                let p = pair.clone().into_inner().next().unwrap();
                self.read_type_spec(scope, &p)
            }
        }
    }

    /// declarator = { array_declarator | simple_declarator }
    /// array_declarator = { identifier ~ fixed_array_size+ }
    /// simple_declarator = { identifier }
    pub fn read_struct_member_declarator(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
        type_spec: &Box<IdlTypeSpec>,
    ) -> Result<Box<IdlStructMember>, IdlError> {
        let decl = pair.clone().into_inner().next().unwrap();

        let mut iter = decl.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("should be declarator {:?}", decl.as_rule());
        }
        match decl.as_rule() {
            // simple_declarator = { identifier }
            Rule::simple_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let member_dcl = Box::new(IdlStructMember {
                    id: id,
                    type_spec: type_spec.clone(),
                    is_key: false,
                });

                Ok(member_dcl)
            }

            // array_declarator = { identifier ~ fixed_array_size+ }
            Rule::array_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let array_sizes: Result<Vec<_>, IdlError> = iter
                    .map(|p|
                            // skip node Rule::fixed_array_size and read const_expr underneath
                            self.read_const_expr(
                                scope,
                                &p.clone().into_inner().next().unwrap()))
                    .collect();
                let array_type_spec =
                    Box::new(IdlTypeSpec::ArrayType(type_spec.clone(), array_sizes?));

                let member_dcl = Box::new(IdlStructMember {
                    id: id,
                    type_spec: array_type_spec,
                    is_key: false,
                });

                Ok(member_dcl)
            }

            _ => Err(IdlError::InternalError),
        }
    }

    // member = { type_spec ~ declarators ~ ";" }
    // declarators = { declarator ~ ("," ~ declarator )* }
    // declarator = { array_declarator | simple_declarator }
    fn read_struct_member(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Vec<Box<IdlStructMember>>, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        let type_spec: Box<IdlTypeSpec> = self.read_type_spec(scope, &iter.next().unwrap())?;

        // skip rule 'declarators' and parse sibblings `declarator'
        let declarators = iter.next().unwrap().clone().into_inner();

        let members: Result<Vec<Box<IdlStructMember>>, IdlError> = declarators
            .map(|declarator| self.read_struct_member_declarator(scope, &declarator, &type_spec))
            .collect();

        members
    }

    //
    fn read_identifier(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<String, IdlError> {
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        match pair.as_rule() {
            Rule::identifier | Rule::enumerator => Ok(pair.as_str().to_owned()),
            _ => Err(IdlError::ExpectedItem(Rule::identifier)),
        }
    }

    /// scoped_name = { "::"? ~ identifier ~ ("::" ~ identifier)* }
    fn read_scoped_name(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<IdlScopedName, IdlError> {
        let iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!(">>> {:?} '{}'", pair.as_rule(), pair.as_str());
        }
        // check if name starts with "::"
        let is_absolute_name = pair.as_str().starts_with("::");
        let scoped_name = iter
            .map(|p| self.read_identifier(scope, &p).unwrap().to_owned())
            .collect::<Vec<String>>();

        Ok(IdlScopedName(scoped_name, is_absolute_name))
    }

    /// const_expr = { unary_expr ~ (or_expr | xor_expr | and_expr | shift_expr | add_expr | mult_expr)? }
    fn read_const_expr(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Box<IdlValueExpr>, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?} '{}'", pair.as_rule(), pair.as_str());
        }
        let fp_collect_init = (None, None, None, None);

        let fp_collect = |(i, f, e, s), node: Pair<Rule>| match node.as_rule() {
            Rule::integral_part => (Some(node.as_str().to_owned()), f, e, s),
            Rule::fractional_part => (i, Some(node.as_str().to_owned()), e, s),
            Rule::exponent => (i, f, Some(node.as_str().to_owned()), s),
            Rule::float_suffix => (i, f, e, Some(node.as_str().to_owned())),
            _ => panic!(),
        };

        match pair.as_rule() {
            Rule::const_expr => match (iter.next(), iter.next()) {
                (Some(ref expr1), Some(ref expr2)) => {
                    let e1 = self.read_const_expr(scope, &expr1)?;
                    let e2 = self.read_const_expr(scope, &expr2)?;
                    Ok(Box::new(IdlValueExpr::Expr(e1, e2)))
                }
                (Some(ref expr1), None) => self.read_const_expr(scope, &expr1),
                _ => Err(IdlError::ExpectedItem(Rule::const_expr)),
            },
            Rule::unary_expr => match (iter.next(), iter.next()) {
                (Some(ref unary_op), Some(ref prim_expr)) => {
                    // TBD
                    let expr = self.read_const_expr(scope, prim_expr)?;
                    match unary_op.as_str() {
                        "-" => Ok(Box::new(IdlValueExpr::UnaryOp(UnaryOp::Neg, expr))),
                        "+" => Ok(Box::new(IdlValueExpr::UnaryOp(UnaryOp::Pos, expr))),
                        "~" => Ok(Box::new(IdlValueExpr::UnaryOp(UnaryOp::Inverse, expr))),
                        _ => Err(IdlError::ExpectedItem(Rule::unary_operator)),
                    }
                }
                (Some(ref prim_expr), None) => self.read_const_expr(scope, prim_expr),
                _ => Err(IdlError::ExpectedItem(Rule::primary_expr)),
            },
            Rule::primary_expr => match iter.next() {
                //  scoped_name = { "::"? ~ identifier ~ ("::" ~ identifier)* }
                Some(ref p) if p.as_rule() == Rule::scoped_name => {
                    let name = self.read_scoped_name(scope, p)?;
                    Ok(Box::new(IdlValueExpr::ScopedName(name)))
                }
                Some(ref p) if p.as_rule() == Rule::literal => self.read_const_expr(scope, p),
                Some(ref p) if p.as_rule() == Rule::const_expr => {
                    let expr = self.read_const_expr(scope, p)?;
                    Ok(Box::new(IdlValueExpr::Brace(expr)))
                }
                _ => Err(IdlError::ExpectedItem(Rule::primary_expr)),
            },
            Rule::and_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::And, expr)))
            }
            Rule::or_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Or, expr)))
            }
            Rule::xor_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Xor, expr)))
            }
            Rule::lshift_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::LShift, expr)))
            }
            Rule::rshift_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::RShift, expr)))
            }
            Rule::add_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Add, expr)))
            }
            Rule::sub_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Sub, expr)))
            }
            Rule::mul_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Mul, expr)))
            }
            Rule::div_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Div, expr)))
            }
            Rule::mod_expr => {
                let expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                Ok(Box::new(IdlValueExpr::BinaryOp(BinaryOp::Mod, expr)))
            }
            Rule::decimal_integer_literal => {
                Ok(Box::new(IdlValueExpr::DecLiteral(pair.as_str().to_owned())))
            }
            Rule::octal_integer_literal => {
                Ok(Box::new(IdlValueExpr::OctLiteral(pair.as_str().to_owned())))
            }
            Rule::hex_integer_literal => {
                Ok(Box::new(IdlValueExpr::HexLiteral(pair.as_str().to_owned())))
            }
            Rule::floating_pt_literal => {
                let (i, f, e, s) = iter.fold(fp_collect_init, fp_collect);
                Ok(Box::new(IdlValueExpr::FloatLiteral(i, f, e, s)))
            }
            Rule::boolean_literal => match pair.as_str() {
                "TRUE" => Ok(Box::new(IdlValueExpr::BooleanLiteral(true))),
                _ => Ok(Box::new(IdlValueExpr::BooleanLiteral(false))),
            },
            Rule::character_literal => Ok(Box::new(IdlValueExpr::CharLiteral(
                pair.as_str().to_owned(),
            ))),
            Rule::wide_character_literal => Ok(Box::new(IdlValueExpr::WideCharLiteral(
                pair.as_str().to_owned(),
            ))),
            Rule::string_literal => Ok(Box::new(IdlValueExpr::StringLiteral(
                pair.as_str().to_owned(),
            ))),
            Rule::wide_string_literal => Ok(Box::new(IdlValueExpr::WideStringLiteral(
                pair.as_str().to_owned(),
            ))),
            _ => self.read_const_expr(scope, &iter.next().unwrap()),
        }
    }

    /// declarator = { array_declarator | simple_declarator }
    /// array_declarator = { identifier ~ fixed_array_size+ }
    /// simple_declarator = { identifier }
    pub fn process_declarator(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
        type_spec: &Box<IdlTypeSpec>,
    ) -> Result<(), IdlError> {
        let decl = pair.clone().into_inner().next().unwrap();
        let mut iter = decl.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", decl.as_rule());
        }
        match decl.as_rule() {
            // simple_declarator = { identifier }
            Rule::simple_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;

                let type_dcl = Box::new(IdlTypeDcl(IdlTypeDclKind::TypeDcl(
                    id.clone(),
                    type_spec.clone(),
                )));
                self.add_type_dcl(scope, id, type_dcl)
            }

            // array_declarator = { identifier ~ fixed_array_size+ }
            Rule::array_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let key = id.clone();

                let array_sizes: Result<Vec<_>, IdlError> = iter
                    .map(|p|
                            // skip node Rule::fixed_array_size and read const_expr underneath
                            self.read_const_expr(
                                scope,
                                &p.clone().into_inner().next().unwrap()))
                    .collect();
                let array_type_spec =
                    Box::new(IdlTypeSpec::ArrayType(type_spec.clone(), array_sizes?));
                let type_dcl = Box::new(IdlTypeDcl(IdlTypeDclKind::TypeDcl(id, array_type_spec)));
                self.add_type_dcl(scope, key, type_dcl)
            }

            // traverse deeper
            _ => Err(IdlError::InternalError),
        }
    }

    ///
    pub fn process<L: IdlLoader>(
        &mut self,
        scope: &mut Scope,
        loader: &mut dyn IdlLoader,
        pair: &Pair<Rule>,
    ) -> Result<(), IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        match pair.as_rule() {
            // module_dcl = { "module" ~ identifier ~ "{" ~ definition* ~ "}" }
            Rule::module_dcl => {
                let id = iter.next().unwrap().as_str();

                scope.push(id.to_owned());

                let _ = self.lookup_module(scope);

                for p in iter {
                    let _ = self.process::<L>(scope, loader, &p);
                }

                let _ = scope.pop();

                Ok(())
            }

            // struct_def = { "struct" ~ identifier ~ (":" ~ scoped_name)? ~ "{" ~ member* ~ "}" }
            Rule::struct_def => {
                let id = iter.next().unwrap().as_str().to_owned();
                let key = id.clone();
                let m1: Result<Vec<Vec<Box<IdlStructMember>>>, _> = iter
                    .map(|p| {
                        // skip hte member-node and read sibbling directly
                        self.read_struct_member(scope, &p)
                    })
                    .collect();

                let m2 = m1?;
                let members = m2.into_iter().flatten().collect::<Vec<_>>();

                let typedcl = Box::new(IdlTypeDcl(IdlTypeDclKind::StructDcl(id, members, false)));
                self.add_type_dcl(scope, key, typedcl)
            }

            // union_def = { "union" ~ identifier ~ "switch" ~ "(" ~ switch_type_spec ~ ")" ~ "{" ~ switch_body ~ "}" }
            Rule::union_def => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let key = id.to_owned();
                let switch_type_spec = self.read_switch_type_spec(scope, &iter.next().unwrap())?;
                let switch_body = self.read_switch_body(scope, &iter.next().unwrap())?;
                let union_def = Box::new(IdlTypeDcl(IdlTypeDclKind::UnionDcl(
                    id,
                    switch_type_spec,
                    switch_body,
                )));

                self.add_type_dcl(scope, key, union_def)
            }

            // type_declarator = { (template_type_spec | constr_type_dcl | simple_type_spec) ~ any_declarators }
            Rule::type_declarator => {
                let type_spec = self.read_type_spec(scope, &iter.next().unwrap())?;

                let any_declarators_pair = &iter.next().unwrap();

                for p in any_declarators_pair.clone().into_inner() {
                    let _ = self.process_declarator(scope, &p, &type_spec);
                }
                Ok(())
            }

            // enum_dcl = { "enum" ~ identifier ~ "{" ~ enumerator ~ ("," ~ enumerator)* ~ ","? ~ "}" }
            // enumerator = { identifier }
            Rule::enum_dcl => {
                let id = iter.next().unwrap().as_str().to_owned();
                let key = id.clone();
                let enums: Result<Vec<_>, IdlError> =
                    iter.map(|p| self.read_identifier(scope, &p)).collect();

                let typedcl = Box::new(IdlTypeDcl(IdlTypeDclKind::EnumDcl(id, enums?)));
                self.add_type_dcl(scope, key, typedcl)
            }
            // const_dcl = { "const" ~ const_type ~ identifier ~ "=" ~ const_expr }
            Rule::const_dcl => {
                let type_spec = self.read_type_spec(scope, &iter.next().unwrap())?;
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let key = id.clone();
                let const_expr = self.read_const_expr(scope, &iter.next().unwrap())?;
                let const_dcl = Box::new(IdlConstDcl {
                    id: id,
                    typedcl: type_spec,
                    value: const_expr,
                });
                self.add_const_dcl(scope, key, const_dcl)
            }

            // include_directive = !{ "#" ~ "include" ~ (("<" ~ path_spec ~ ">") | ("\"" ~ path_spec ~ "\"")) }
            Rule::include_directive => {
                match pair.clone().into_inner().nth(0) {
                    Some(ref p) => {
                        let fname = p.as_str();
                        let data = loader
                            .load(fname)
                            .map_err(|_| IdlError::FileNotFound(fname.to_owned()))?;

                        let idl: Pairs<Rule> = IdlParser::parse(Rule::specification, &data)
                            .map_err(|e| IdlError::ErrorMesg(e.to_string()))?;

                        for p in idl {
                            self.process::<L>(scope, loader, &p)?;
                        }
                    }
                    _ => {}
                }
                Ok(())
            }

            Rule::other_directive => self.add_other_directive(scope, pair.as_str()),

            // anything else
            _ => {
                for p in iter {
                    let _ = self.process::<L>(scope, loader, &p);
                }
                Ok(())
            }
        }
    }

    /// declarator = { array_declarator | simple_declarator }
    /// array_declarator = { identifier ~ fixed_array_size+ }
    /// simple_declarator = { identifier }
    pub fn read_switch_element_declarator(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
        type_spec: &Box<IdlTypeSpec>,
    ) -> Result<Box<IdlSwitchElement>, IdlError> {
        let decl = pair.clone().into_inner().next().unwrap();

        let mut iter = decl.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("should be declarator {:?}", decl.as_rule());
        }
        match decl.as_rule() {
            // simple_declarator = { identifier }
            Rule::simple_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let member_dcl = Box::new(IdlSwitchElement {
                    id: id,
                    type_spec: type_spec.clone(),
                });

                Ok(member_dcl)
            }

            // array_declarator = { identifier ~ fixed_array_size+ }
            Rule::array_declarator => {
                let id = self.read_identifier(scope, &iter.next().unwrap())?;
                let array_sizes: Result<Vec<_>, IdlError> = iter
                    .map(|p|
                            // skip node Rule::fixed_array_size and read const_expr underneath
                            self.read_const_expr(
                                scope,
                                &p.clone().into_inner().next().unwrap()))
                    .collect();
                let array_type_spec =
                    Box::new(IdlTypeSpec::ArrayType(type_spec.clone(), array_sizes?));

                let member_dcl = Box::new(IdlSwitchElement {
                    id: id,
                    type_spec: array_type_spec,
                });

                Ok(member_dcl)
            }

            _ => Err(IdlError::InternalError),
        }
    }

    /// element_spec = { type_spec ~ declarator }
    fn read_switch_element_spec(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Box<IdlSwitchElement>, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        let type_spec: Box<IdlTypeSpec> = self.read_type_spec(scope, &iter.next().unwrap())?;

        let element_spec =
            self.read_switch_element_declarator(scope, &iter.next().unwrap(), &type_spec);

        element_spec
    }

    /// switch_type_spec = {integer_type | char_type | boolean_type | wide_char_type| octet_type| scoped_name }
    fn read_switch_type_spec(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Box<IdlTypeSpec>, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }
        self.read_type_spec(scope, &iter.next().unwrap())
    }

    /// switch_body = { case+ }
    fn read_switch_body(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<Vec<IdlSwitchCase>, IdlError> {
        let iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }

        let cases: Result<Vec<_>, IdlError> =
            iter.map(|p| self.read_switch_case(scope, &p)).collect();

        cases
    }

    /// case_label = { "case" ~ const_expr ~ ":" | "default" ~ ":" }
    fn read_switch_label(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<IdlSwitchLabel, IdlError> {
        let mut iter = pair.clone().into_inner();
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }

        match iter.next() {
            Some(p) => {
                let expr = self.read_const_expr(scope, &p)?;
                Ok(IdlSwitchLabel::Label(expr))
            }
            _ => Ok(IdlSwitchLabel::Default),
        }
    }

    /// case = { case_label+ ~ element_spec ~ ";" }
    fn read_switch_case(
        &mut self,
        scope: &mut Scope,
        pair: &Pair<Rule>,
    ) -> Result<IdlSwitchCase, IdlError> {
        if self.config.verbose {
            print!("{:indent$}", "", indent = 3 * scope.len());
            println!("{:?}", pair.as_rule());
        }

        let case_labels: Result<Vec<IdlSwitchLabel>, IdlError> = pair
            .clone()
            .into_inner()
            .filter(|p| p.as_rule() == Rule::case_label)
            .map(|p| self.read_switch_label(scope, &p))
            .collect();

        // there will be only one in the list, choose the last
        let elem_spec: Result<Box<IdlSwitchElement>, IdlError> = pair
            .clone()
            .into_inner()
            .filter(|p| p.as_rule() == Rule::element_spec)
            .map(|p| self.read_switch_element_spec(scope, &p))
            .last()
            .unwrap();

        Ok(IdlSwitchCase {
            labels: case_labels?,
            elem_spec: elem_spec?,
        })
    }

    // enum_dcl = { "enum" ~ identifier ~ "{" ~ enumerator ~ ("," ~ enumerator)* ~ ","? ~ "}" }
    // enumerator = { identifier }
}

///
///
pub fn generate_with_loader<W: Write, L: IdlLoader>(
    out: &mut W,
    loader: &mut L,
    config: &Configuration,
    idldecl: &str,
) -> Result<(), IdlError> {
    let mut ctx = Context::new(config);

    let idl: Pairs<Rule> = IdlParser::parse(Rule::specification, &idldecl)
        .map_err(|e| IdlError::ErrorMesg(e.to_string()))?;

    let mut scope = Scope::new();

    for p in idl {
        let _ = ctx.process::<L>(&mut scope, loader, &p);
    }

    if config.generate_c {
        let idlnamestem = String::from(config.idl_name.file_stem().unwrap().to_str().unwrap());
        let idlname = config.idl_name.file_name().unwrap().to_str().unwrap();

        let mut header_name = idlnamestem.clone();
        header_name.push_str(".h");

        let file_header = std::include_str!("cdds/templates/file_header.txt")
            .replace("<FILENAME>", &header_name)
            .replace("<IDLNAME>", idlname)
            .replace(
                "<HEADERDEFINE>",
                &crate::cdds::header_macro_name(&idlnamestem),
            );
        let _ = out.write(file_header.as_bytes());

        let hfile_footer = std::include_str!("cdds/templates/h_file_footer.txt").replace(
            "<HEADERDEFINE>",
            &crate::cdds::header_macro_name(&idlnamestem),
        );

        let scope = Vec::new();

        ctx.root_module
            .write_h(out, &scope, &ctx.root_module)
            .map_err(|_| IdlError::InternalError)?;

        out.write(hfile_footer.as_bytes())
            .map(|_| ())
            .map_err(|e| IdlError::ErrorMesg(e.to_string()))
    } else {
        let _ = out.write(MODULE_PRELUDE);
        let USE_CYCLONEDDS_SYS = include_str!("templates/use_cyclonedds.txt");
        let _ = out.write(USE_CYCLONEDDS_SYS.as_bytes());
        ctx.root_module
            .write(out, 0, &ctx.root_module,&Vec::new(), config.generate_descriptors)
            .map_err(|_| IdlError::InternalError)
    }
}

#[derive(Debug, Clone, Default)]
struct Loader {
    search_path: Vec<String>,
}

///
fn load_from(prefix: &std::path::Path, filename: &str) -> Result<String, Error> {
    let fullname = prefix.join(filename);

    let mut file = File::open(fullname)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    return Ok(data);
}

///
impl Loader {
    pub fn new(search_path: Vec<String>) -> Loader {
        Loader {
            search_path: search_path,
        }
    }
}

///
impl IdlLoader for Loader {
    fn load(&self, filename: &str) -> Result<String, Error> {
        for prefix in &self.search_path {
            let prefix_path = std::path::Path::new(&prefix);
            match load_from(&prefix_path, filename) {
                Ok(data) => return Ok(data),
                _ => continue,
            }
        }
        Err(Error::from(ErrorKind::NotFound))
    }
}

///
pub fn generate_with_search_path<W: Write>(
    out: &mut W,
    search_path: Vec<String>,
    config: &Configuration,
    data: &str,
) -> Result<(), IdlError> {
    let mut loader = Loader::new(search_path);

    generate_with_loader(out, &mut loader, config, data)
}
