use circt_sys::*;

use crate::crate_prelude::*;

use super::Context;

wrap_raw_ptr!(DialectHandle);

impl DialectHandle {
    fn register_dialect(&self, ctx: &Context) {
        unsafe { mlirDialectHandleRegisterDialect(self.raw(), ctx.raw()) }
    }

    pub fn load(&self, ctx: &Context) -> Option<Dialect> {
        self.register_dialect(ctx);
        Dialect::try_from_raw(unsafe { mlirDialectHandleLoadDialect(self.raw(), ctx.raw()) })
    }
}

wrap_raw_ptr!(Dialect);

impl PartialEq for Dialect {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirDialectEqual(self.raw(), other.raw()) }
    }
}

impl std::fmt::Debug for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Dialect").field(&self.0).finish()
    }
}
