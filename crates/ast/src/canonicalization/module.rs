#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use bumpalo::Bump;
use broc_collections::all::{default_hasher, ImMap, ImSet, MutMap, MutSet, SendMap};
use broc_module::ident::Ident;
use broc_module::ident::Lowercase;
use broc_module::symbol::IdentIdsByModule;
use broc_module::symbol::{IdentIds, ModuleId, ModuleIds, Symbol};
use broc_parse::ast;
use broc_parse::pattern::PatternType;
use broc_problem::can::{Problem, RuntimeError};
use broc_region::all::{Loc, Region};
use broc_types::subs::{VarStore, Variable};

use crate::lang::core::def::def::canonicalize_defs;
use crate::lang::core::def::def::Def;
use crate::lang::core::def::def::{sort_can_defs, Declaration};
use crate::lang::core::expr::expr2::Expr2;
use crate::lang::core::expr::output::Output;
use crate::lang::core::pattern::Pattern2;
use crate::lang::core::types::Alias;
use crate::lang::core::val_def::ValueDef;
use crate::lang::env::Env;
use crate::lang::scope::Scope;
use crate::mem_pool::pool::NodeId;
use crate::mem_pool::pool::Pool;
use crate::mem_pool::pool_vec::PoolVec;
use crate::mem_pool::shallow_clone::ShallowClone;

pub struct ModuleOutput {
    pub aliases: MutMap<Symbol, NodeId<Alias>>,
    pub rigid_variables: MutMap<Variable, Lowercase>,
    pub declarations: Vec<Declaration>,
    pub exposed_imports: MutMap<Symbol, Variable>,
    pub lookups: Vec<(Symbol, Variable, Region)>,
    pub problems: Vec<Problem>,
    pub ident_ids: IdentIds,
    pub references: MutSet<Symbol>,
}
