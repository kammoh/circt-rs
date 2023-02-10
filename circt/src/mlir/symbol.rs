use crate::crate_prelude::*;
use circt_sys::*;

use lazy_static::lazy_static;

wrap_raw_ptr!(SymbolTable);
impl_into_owned!(SymbolTable);

lazy_static! {
    static ref SYMBOL_ATTR_NAME: String = {
        let sr = StringRef::from_raw(unsafe { mlirSymbolTableGetSymbolAttributeName() });
        String::from(sr.as_str())
    };
    static ref VISIBILITY_ATTR_NAME: String = {
        let sr = StringRef::from_raw(unsafe { mlirSymbolTableGetVisibilityAttributeName() });
        String::from(sr.as_str())
    };
}

impl SymbolTable {
    /// Creates a symbol table for the given operation.
    /// If the operation does not have the SymbolTable trait, returns a None.
    pub fn create(op: &impl Op) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirSymbolTableCreate(op.raw()) })
    }

    /// Returns the name of the attribute used to store symbol names compatible with symbol tables.
    pub fn symbol_attr_name() -> &'static str {
        SYMBOL_ATTR_NAME.as_str()
    }

    /// Returns the name of the attribute used to store symbol visibility.
    pub fn visibility_attribute_name() -> &'static str {
        VISIBILITY_ATTR_NAME.as_str()
    }

    /// Looks up a symbol with the given name in the given symbol table and returns the operation that corresponds to the symbol.
    /// If the symbol cannot be found, returns None.
    pub fn lookup(&self, name: &str) -> Option<Operation> {
        let sr = StringRef::from_str(name);
        Operation::try_from_raw(unsafe { mlirSymbolTableLookup(self.raw(), sr.raw()) })
    }

    /// Inserts the given operation into the given symbol table.
    /// The operation must have the symbol trait.
    /// If the symbol table already has a symbol with the same name, renames the symbol being inserted to ensure name uniqueness.
    /// Note that this does not move the operation itself into the block of the symbol table operation, this should be done separately.
    /// Returns the name of the symbol after insertion.
    pub fn insert(&self, op: &impl Op) -> Option<Attribute> {
        Attribute::try_from_raw(unsafe { mlirSymbolTableInsert(self.raw(), op.raw()) })
    }
}
