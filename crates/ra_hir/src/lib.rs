//! HIR (previously known as descriptors) provides a high-level object oriented
//! access to Rust code.
//!
//! The principal difference between HIR and syntax trees is that HIR is bound
//! to a particular crate instance. That is, it has cfg flags and features
//! applied. So, the relation between syntax and HIR is many-to-one.

#![recursion_limit = "512"]

macro_rules! impl_froms {
    ($e:ident: $($v:ident $(($($sv:ident),*))?),*) => {
        $(
            impl From<$v> for $e {
                fn from(it: $v) -> $e {
                    $e::$v(it)
                }
            }
            $($(
                impl From<$sv> for $e {
                    fn from(it: $sv) -> $e {
                        $e::$v($v::$sv(it))
                    }
                }
            )*)?
        )*
    }
}

mod either;
pub mod debug;

pub mod db;
#[macro_use]
pub mod mock;
mod path;
pub mod source_binder;

mod source_id;
mod ids;
mod name;
mod nameres;
mod adt;
mod traits;
mod type_alias;
mod type_ref;
mod ty;
mod attr;
mod impl_block;
mod expr;
mod lang_item;
mod generics;
mod resolve;
pub mod diagnostics;
mod util;

mod code_model;

pub mod from_source;

#[cfg(test)]
mod marks;

use crate::{
    ids::MacroFileKind,
    name::AsName,
    resolve::Resolver,
    source_id::{AstId, FileAstId},
};

pub use self::{
    adt::VariantDef,
    either::Either,
    expr::ExprScopes,
    from_source::FromSource,
    generics::{GenericDef, GenericParam, GenericParams, HasGenericParams},
    ids::{HirFileId, MacroCallId, MacroCallLoc, MacroDefId, MacroFile},
    impl_block::ImplBlock,
    name::Name,
    nameres::{ImportId, Namespace, PerNs},
    path::{Path, PathKind},
    resolve::ScopeDef,
    source_binder::{PathResolution, ScopeEntryWithSyntax, SourceAnalyzer},
    source_id::{AstIdMap, ErasedFileAstId},
    ty::{
        display::HirDisplay, ApplicationTy, CallableDef, Substs, TraitRef, Ty, TypeCtor, TypeWalk,
    },
    type_ref::Mutability,
};

pub use self::code_model::{
    docs::{DocDef, Docs, Documentation},
    src::{HasBodySource, HasSource, Source},
    Adt, AssocItem, BuiltinType, Const, ConstData, Container, Crate, CrateDependency, DefWithBody,
    Enum, EnumVariant, FieldSource, FnData, Function, HasBody, MacroDef, Module, ModuleDef,
    ModuleSource, Static, Struct, StructField, Trait, TypeAlias, Union,
};
