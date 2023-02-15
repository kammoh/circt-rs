use circt_sys::*;

use crate::crate_prelude::*;

wrap_raw_ptr!(PassManager);
impl_into_owned!(PassManager);

impl PassManager {
    /// Create a new top-level PassManager with the default anchor.
    pub fn new(ctx: &Context) -> Self {
        Self::try_from_raw(unsafe { mlirPassManagerCreate(ctx.raw()) }).unwrap()
    }
    /// Create a new top-level PassManager anchored on `anchor_op`
    pub fn new_anchored_on(ctx: &Context, anchor_op: &str) -> Self {
        Self::try_from_raw(unsafe {
            mlirPassManagerCreateOnOperation(ctx.raw(), StringRef::from_str(anchor_op).raw())
        })
        .unwrap()
    }

    /// Add a pass and transfer ownership to the provided top-level mlirPassManager.
    /// If the pass is not a generic operation pass or a ModulePass, a new OpPassManager is
    ///  implicitly nested under the provided PassManager.
    pub fn add_pass(&self, pass: &Pass) -> &Self {
        unsafe { mlirPassManagerAddOwnedPass(self.raw(), pass.raw()) };
        self
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
    pub fn run(&self, module: &Module) -> Result<(), Error> {
        LogicalResult::from_raw(unsafe { mlirPassManagerRun(self.raw(), module.raw()) })
            .to_result((), Error::PassManagerRunFailure(module.op().name().to_string()))
    }

    /// Parse a sequence of textual MLIR pass pipeline elements and add them to self.
    pub fn parse_pass(&self, pipeline: &str) -> Result<&Self, Error> {
        let opm =
            OpPassManager::try_from_raw(unsafe { mlirPassManagerGetAsOpPassManager(self.raw()) })
                .ok_or(Error::simple(format!("Parsing of pipeline: '{}' failed", pipeline)))?;
        opm.parse_pass(pipeline)?;
        Ok(self)
    }

    /// Parse a textual MLIR pass pipeline and assign it to the self.
    /// Pipeline must be wrapped with the anchor operation type, e.g. 'builtin.module(...)'"
    pub fn parse(&self, pipeline: &str) -> Result<&Self, Error> {
        let opm: OpPassManager = self.into();
        opm.parse(pipeline)?;
        Ok(self)
    }

    /// Nest an OpPassManager under the top-level PassManager, the nested passmanager will only run on operations matching the provided name.
    /// The returned OpPassManager will be destroyed when the parent is destroyed.
    /// To further nest more OpPassManager under the newly returned one, see OpPassManager::nest() below.
    pub fn nest(&self, operation_name: &str) -> OpPassManager {
        let operation_name = StringRef::from_str(operation_name);
        OpPassManager::try_from_raw(unsafe {
            mlirPassManagerGetNestedUnder(self.raw(), operation_name.raw())
        })
        .unwrap()
    }

    pub fn print(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let opm: OpPassManager = self.into();
        opm.print(f)
    }
}

impl From<&PassManager> for OpPassManager {
    fn from(value: &PassManager) -> Self {
        OpPassManager::try_from_raw(unsafe { mlirPassManagerGetAsOpPassManager(value.raw()) })
            .unwrap()
    }
}

impl std::fmt::Debug for PassManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}

wrap_raw_ptr!(OpPassManager);

impl OpPassManager {
    /// Parse a sequence of textual MLIR pass pipeline elements and add them to this OpPassManager.
    /// (nested)
    pub fn parse_pass(&self, pipeline: &str) -> Result<&Self, Error> {
        let pipeline = StringRef::from_str(pipeline);

        let mut error_message = String::new();
        let fmt = FormatterCallback::new(&mut error_message);
        LogicalResult::from_raw(unsafe {
            mlirOpPassManagerAddPipeline(self.0, pipeline.raw(), fmt.callback(), fmt.user_data())
        })
        .to_option(self)
        .ok_or(Error::simple(error_message))
    }

    /// Parse a textual MLIR pass pipeline and assign it to the provided OpPassManager.
    pub fn parse(&self, pipeline: &str) -> Result<&Self, Error> {
        let pipeline = StringRef::from_str(pipeline);

        let mut err = String::new();
        let fmt = FormatterCallback::new(&mut err);
        LogicalResult::from_raw(unsafe {
            mlirParsePassPipeline(self.0, pipeline.raw(), fmt.callback(), fmt.user_data())
        })
        .to_option(self)
        .ok_or(Error::simple(err))
    }

    /// Add a pass and transfer ownership to the provided mlirOpPassManager.
    /// If the pass is not a generic operation pass or matching the type of the provided PassManager,
    ///  a new OpPassManager is implicitly nested under the provided PassManager.
    pub fn add_pass(&self, pass: &Pass) -> &Self {
        unsafe { mlirOpPassManagerAddOwnedPass(self.raw(), pass.raw()) };
        self
    }

    /// Nest an OpPassManager under the provided OpPassManager, the nested passmanager will only run on operations matching the provided name.
    /// The returned OpPassManager will be destroyed when the parent is destroyed.
    pub fn nest(&self, operation_name: &str) -> OpPassManager {
        let operation_name = StringRef::from_str(operation_name);
        OpPassManager::try_from_raw(unsafe {
            mlirOpPassManagerGetNestedUnder(self.raw(), operation_name.raw())
        })
        .unwrap()
    }

    pub fn print(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let fmt = FormatterCallback::new(f);
        unsafe { mlirPrintPassPipeline(self.raw(), fmt.callback(), fmt.user_data()) };
        Ok(())
    }
}

impl std::fmt::Debug for OpPassManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}

impl Owned<PassManager> {
    pub fn new(ctx: &Context) -> Self {
        Self(PassManager::new(ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_pm() {
        let ctx = OwnedContext::default();
        let pm = PassManager::new(&ctx);
        hw::dialect().load(&ctx).unwrap();
        hw::register_arith_passes();
        pm.parse("asdfasdfasf").expect_err("should fail");
        pm.parse("builtin.module(lower-hwarith-to-hw)").expect("should succeed");

        let loc = Location::new_unknown(&ctx);

        let module = Module::create(&loc);
        for pipeline in &["builtin.module(lower-hwarith-to-hw)"] {
            pm.parse(pipeline).expect("parse failed");
        }
        pm.run(&module).unwrap();

        let out_dir = Path::new("test_module");
        std::fs::create_dir_all(out_dir).unwrap();
        sv::export_split_verilog(&module, &out_dir);
    }
}
