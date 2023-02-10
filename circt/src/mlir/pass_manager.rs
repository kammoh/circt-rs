use circt_sys::*;
use simple_error::SimpleError;

use crate::crate_prelude::*;

wrap_raw!(LogicalResult);

impl LogicalResult {
    pub fn to_option<T>(&self, ok: T) -> Option<T> {
        self.is_success().then_some(ok)
    }

    pub fn is_success(&self) -> bool {
        self.raw().value != 0
    }
}


wrap_raw_ptr!(OpPassManager);

impl OpPassManager {
    /// Parse a textual MLIR pass pipeline and assign it to the provided OpPassManager.
    pub fn parse(&self, pipeline: &str) -> Result<(), SimpleError> {
        let pipeline = StringRef::from_str(pipeline);

        let mut err = String::new();
        let fmt = FormatterCallback::new(&mut err);
        println!("OpPassManager::parse 2");
        LogicalResult::from_raw(unsafe {
            mlirParsePassPipeline(self.0, pipeline.raw(), fmt.callback(), fmt.user_data())
        })
        .to_option(())
        .ok_or(SimpleError::new(err))
    }

    /// Parse a sequence of textual MLIR pass pipeline elements and add them to this OpPassManager.
    pub fn add(&self, pipeline: &str) -> Result<(), SimpleError> {
        let pipeline = StringRef::from_str(pipeline);

        let mut err = String::new();
        let fmt = FormatterCallback::new(&mut err);
        LogicalResult::from_raw(unsafe {
            mlirOpPassManagerAddPipeline(self.0, pipeline.raw(), fmt.callback(), fmt.user_data())
        })
        .to_option(())
        .ok_or(SimpleError::new(err))
    }
}

wrap_raw_ptr!(PassManager);
impl_into_owned!(PassManager);

impl PassManager {
    /// Create a new top-level PassManager with the default anchor.
    pub fn new(ctx: &Context) -> Self {
        Self::try_from_raw(unsafe { mlirPassManagerCreate(ctx.raw()) }).unwrap()
    }

    /// Enable mlir-print-ir-after-all.
    pub fn enable_ir_printing(&self) {
        unsafe { mlirPassManagerEnableIRPrinting(self.0) }
    }

    /// Enable / disable verify-each.
    pub fn enable_verifier(&self, enable: bool) {
        unsafe { mlirPassManagerEnableVerifier(self.0, enable) }
    }

    /// Run the PassManager on the given module
    pub fn run(&self, module: &Module) -> LogicalResult {
        LogicalResult::from_raw(unsafe { mlirPassManagerRun(self.raw(), module.raw()) })
    }

    /// Parse a textual MLIR pass pipeline and assign it to the self.
    pub fn parse(&self, pipeline: &str) -> Result<(), SimpleError> {
        let pm =
            OpPassManager::try_from_raw(unsafe { mlirPassManagerGetAsOpPassManager(self.raw()) })
                .ok_or(SimpleError::new("failed"))?;
        pm.parse(pipeline)
    }

    /// Parse a sequence of textual MLIR pass pipeline elements and add them to self.
    pub fn add(&self, pipeline: &str) -> Result<(), SimpleError> {
        let pm =
            OpPassManager::try_from_raw(unsafe { mlirPassManagerGetAsOpPassManager(self.raw()) })
                .ok_or(SimpleError::new("failed"))?;
        pm.add(pipeline)
    }
}

// impl Drop for PassManager {
//     fn drop(&mut self) {
//         println!("drop PassManager");
//         if !self.0.ptr.is_null() {
//             self.destroy();
//             self.0 = MlirPassManager {
//                 ptr: std::ptr::null_mut(),
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{export_split_verilog, hw};

    use super::*;

    #[test]
    fn test_pm() {
        let ctx = OwnedContext::new();
        let pm = PassManager::new(&ctx);
        hw::dialect().unwrap().load_dialect(&ctx).unwrap();
        hw::register_hw_arith_passes();
        pm.parse("asdfasdfasf").expect_err("should fail");
        pm.parse("builtin.module(lower-hwarith-to-hw)")
            .expect("should succeed");

        let loc = Location::new_unknown(&ctx);


        let module = Module::create(&loc);
        for pipeline in &["builtin.module(lower-hwarith-to-hw)"] {
            pm.parse(pipeline).expect("parse failed");
            let r = pm.run(&module);
            assert!(r.is_success());
        }
        let out_dir = Path::new("test_module");
        fs::create_dir_all(out_dir).unwrap();
        export_split_verilog(&module, &out_dir);
    }
}
