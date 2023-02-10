use crate::{crate_prelude::*, ModuleOp};
use circt_sys::*;

wrap_raw_ptr!(Module);

impl Module {
    /// Creates a new, empty module and transfers ownership to the caller.
    pub fn create(location: &Location) -> Self {
        Self::try_from_raw(unsafe { mlirModuleCreateEmpty(location.raw()) }).unwrap()
    }

    /// Parses a module from the string and transfers ownership to the caller.
    pub fn parse(ctx: &Context, pipeline: &str) -> Option<Self> {
        Self::try_from_raw(unsafe {
            mlirModuleCreateParse(ctx.raw(), StringRef::from_str(pipeline).raw())
        })
    }

    /// Views the generic operation as a module.
    /// Returns None when the input operation was not a ModuleOp.
    pub fn from_op(op: &impl HasRaw<RawType = MlirOperation>) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirModuleFromOperation(op.raw()) })
    }

    /// Gets the context that a module was created with.
    pub fn context(&self) -> Context {
        Context::try_from_raw(unsafe { mlirModuleGetContext(self.raw()) }).unwrap()
    }

    /// Gets the body of the module, i.e. the only block it contains.
    pub fn body(&self) -> Block {
        Block::try_from_raw(unsafe { mlirModuleGetBody(self.raw()) }).unwrap()
    }


    /// Views the module as a ModuleOp operation.
    pub fn operation(&self) -> ModuleOp {
        ModuleOp::try_from_raw(unsafe { mlirModuleGetOperation(self.raw()) }).unwrap()
    }
}
