// Copyright (c) 2016-2021 Fabian Schuiki

use std::convert::TryInto;

pub mod attr;
pub mod builder;
pub mod context;
pub mod loc;
pub mod ty;
pub mod value;

pub use attr::*;
pub use builder::*;
use circt_sys::capi::mlirIntegerAttrGetValueInt;
use circt_sys::capi::mlirLocationUnknownGet;
use circt_sys::capi::mlirOpPrintingFlagsCreate;
use circt_sys::capi::mlirOpPrintingFlagsDestroy;
use circt_sys::capi::mlirOpPrintingFlagsEnableDebugInfo;
use circt_sys::capi::mlirOperationCreate;
use circt_sys::capi::mlirOperationDump;
use circt_sys::capi::mlirOperationGetAttributeByName;
use circt_sys::capi::mlirOperationGetBlock;
use circt_sys::capi::mlirOperationGetContext;
use circt_sys::capi::mlirOperationGetRegion;
use circt_sys::capi::mlirOperationGetResult;
use circt_sys::capi::mlirOperationPrintWithFlags;
use circt_sys::capi::mlirOperationStateAddAttributes;
use circt_sys::capi::mlirOperationStateAddOperands;
use circt_sys::capi::mlirOperationStateAddOwnedRegions;
use circt_sys::capi::mlirOperationStateAddResults;
use circt_sys::capi::mlirOperationStateAddSuccessors;
use circt_sys::capi::mlirOperationStateGet;
use circt_sys::capi::mlirOperationVerify;
use circt_sys::capi::mlirRegionGetFirstBlock;
use circt_sys::capi::MlirAttribute;
use circt_sys::capi::MlirBlock;
pub use circt_sys::capi::MlirBlock as Block;
use circt_sys::capi::MlirDialectHandle;
use circt_sys::capi::MlirLocation;
use circt_sys::capi::MlirNamedAttribute;
use circt_sys::capi::MlirOperation;
use circt_sys::capi::MlirOperationState;
use circt_sys::capi::MlirRegion;
pub use circt_sys::capi::MlirRegion as Region;
use circt_sys::capi::MlirStringRef;
use circt_sys::capi::MlirType;
use circt_sys::capi::MlirValue;
pub use context::*;
pub use loc::*;
pub use ty::*;
pub use value::*;

pub struct Owned<T: IntoOwned>(T);

pub trait IntoOwned {
    fn destroy(&mut self);
}

impl<T: IntoOwned> Drop for Owned<T> {
    fn drop(&mut self) {
        self.0.destroy()
    }
}

impl<T: IntoOwned> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// Common facilities for types that wrap an underlying raw MLIR C pointer.
pub trait WrapRaw: Copy {
    type RawType;
    /// Wrap an existing raw MLIR C pointer.
    fn from_raw(raw: Self::RawType) -> Self;
    /// Get the underlying raw MLIR C pointer.
    fn raw(&self) -> Self::RawType;
}

#[derive(Copy, Clone)]
pub struct DialectHandle(MlirDialectHandle);

impl DialectHandle {
    pub fn from_raw(raw: MlirDialectHandle) -> Self {
        Self(raw)
    }
}

/// A trait implemented by anything that wraps an MLIR operation.
pub trait OperationExt: WrapRaw<RawType = MlirOperation> {
    /// Return the full operation name, like `builtin.module`.
    fn operation_name() -> &'static str;

    /// Get the underlying C API operation.
    fn raw_operation(&self) -> MlirOperation {
        self.raw()
    }

    fn operation(&self) -> Operation {
        Operation(self.raw())
    }

    /// Print the operation to stderr.
    fn dump(&self) {
        unsafe { mlirOperationDump(self.raw()) };
    }

    /// Print the operation to anything that implements `std::io::Write`.
    fn print<T: std::io::Write>(&self, mut to: T, with_debug_info: bool) {
        /// Helper callback function that interprets its `user_data` field as a
        /// reference to a reference to something that implements `Write`. The
        /// double reference is required to ensure we're not trying to pass a
        /// fat pointer (e.g. for `T = &dyn Write`) to the C callback.
        unsafe extern "C" fn callback<T: std::io::Write>(
            string: MlirStringRef,
            to: *mut std::ffi::c_void,
        ) {
            let to: &mut &mut T = std::mem::transmute(to);
            to.write(std::slice::from_raw_parts(
                string.data as *const _,
                string.length as usize,
            ))
            .unwrap();
        }

        // Print the operation through the above callback, which basically just
        // forwards the chunks to the Rust-native `Write` implementation.
        unsafe {
            let flags = mlirOpPrintingFlagsCreate();
            if with_debug_info {
                mlirOpPrintingFlagsEnableDebugInfo(flags, false, false);
            }
            mlirOperationPrintWithFlags(
                self.raw(),
                flags,
                Some(callback::<T>),
                (&mut &mut to) as *const _ as *mut _,
            );
            mlirOpPrintingFlagsDestroy(flags);
        }
    }

    /// Verify the operation.
    fn verify(&self) -> bool {
        unsafe { mlirOperationVerify(self.raw()) }
    }

    /// Return the MLIR context for this operation.
    fn context(&self) -> Context {
        unsafe { Context::from_raw(mlirOperationGetContext(self.raw())) }
    }

    /// Return the parent block this operation is in.
    fn parent_block(&self) -> MlirBlock {
        unsafe { mlirOperationGetBlock(self.raw()) }
    }

    /// Return an attribute of the operation.
    fn attr(&self, name: &str) -> MlirAttribute {
        unsafe { mlirOperationGetAttributeByName(self.raw(), name.into()) }
    }

    /// Return an attribute of the operation as an `i64`.
    fn get_attr_i64(&self, name: &str) -> Option<i64> {
        let attr = self.attr(name);
        if attr.ptr.is_null() {
            None
        } else {
            unsafe { Some(mlirIntegerAttrGetValueInt(attr)) }
        }
    }

    /// Return an attribute of the operation as a `i64`.
    fn attr_i64(&self, name: &str) -> i64 {
        self.get_attr_i64(name).unwrap()
    }

    /// Return an attribute of the operation as a `usize`.
    fn get_attr_usize(&self, name: &str) -> Option<usize> {
        self.get_attr_i64(name).and_then(|x| x.try_into().ok())
    }

    /// Return an attribute of the operation as a `usize`.
    fn attr_usize(&self, name: &str) -> usize {
        self.get_attr_usize(name).unwrap()
    }

    /// Get one of the results of the operation.
    fn result(&self, index: usize) -> Value {
        Value::from_raw(unsafe { mlirOperationGetResult(self.raw(), index as _) })
    }
}

/// An operation that has a single region.
pub trait SingleRegionOp: OperationExt {
    /// Get the single body region of this operation.
    fn region(&self) -> MlirRegion {
        unsafe { mlirOperationGetRegion(self.raw(), 0) }
    }

    /// Get the first block in the body region.
    fn first_block(&self) -> MlirBlock {
        unsafe { mlirRegionGetFirstBlock(self.region()) }
    }
}

/// An operation that has a single block in a single region.
pub trait SingleBlockOp: SingleRegionOp {
    /// Get the single body block of this operation.
    fn block(&self) -> MlirBlock {
        unsafe { mlirRegionGetFirstBlock(self.region()) }
    }
}

#[derive(Clone, Copy)]
pub struct Operation(MlirOperation);

impl WrapRaw for Operation {
    type RawType = MlirOperation;
    fn from_raw(raw: MlirOperation) -> Self {
        Self(raw)
    }
    fn raw(&self) -> MlirOperation {
        self.0
    }
}

impl OperationExt for Operation {
    fn operation_name() -> &'static str {
        panic!("unspecific operation")
    }
}

pub struct OperationState(MlirOperationState);

impl OperationState {
    pub fn new(op_name: &str, loc: MlirLocation) -> Self {
        Self(unsafe { mlirOperationStateGet(op_name.into(), loc) })
    }

    pub fn with_unknown_location(cx: Context, op_name: &str) -> Self {
        Self::new(op_name, unsafe { mlirLocationUnknownGet(cx.raw()) })
    }

    pub fn raw(&self) -> MlirOperationState {
        self.0
    }

    pub fn raw_mut(&mut self) -> &mut MlirOperationState {
        &mut self.0
    }

    /// Add a result to the operation.
    pub fn add_result(&mut self, ty: Type) {
        self.add_results(&[ty.raw()])
    }

    /// Add multiple results to the operation.
    pub fn add_results(&mut self, types: &[MlirType]) {
        unsafe { mlirOperationStateAddResults(self.raw_mut(), types.len() as _, types.as_ptr()) }
    }

    /// Add an operand to the operation.
    pub fn add_operand(&mut self, value: Value) {
        self.add_operands(&[value.raw()])
    }

    /// Add multiple operands to the operation.
    pub fn add_operands(&mut self, values: &[MlirValue]) {
        unsafe { mlirOperationStateAddOperands(self.raw_mut(), values.len() as _, values.as_ptr()) }
    }

    /// Add an attribute to the operation.
    pub fn add_attribute(&mut self, name: &str, attr: Attribute) {
        self.add_attributes(&[attr.named(name)]);
    }

    /// Add multiple attributes to the operation.
    pub fn add_attributes(&mut self, attrs: &[MlirNamedAttribute]) {
        unsafe { mlirOperationStateAddAttributes(self.raw_mut(), attrs.len() as _, attrs.as_ptr()) }
    }

    /// Add a successor to the operation.
    pub fn add_successor(&mut self, block: MlirBlock) {
        self.add_successors(&[block]);
    }

    /// Add multiple successors to the operation.
    pub fn add_successors(&mut self, blocks: &[MlirBlock]) {
        unsafe {
            mlirOperationStateAddSuccessors(self.raw_mut(), blocks.len() as _, blocks.as_ptr())
        }
    }

    /// Add a region to the operation.
    pub fn add_region(&mut self, region: MlirRegion) {
        self.add_regions(&[region])
    }

    /// Add multiple regions to the operation.
    pub fn add_regions(&mut self, regions: &[MlirRegion]) {
        unsafe {
            mlirOperationStateAddOwnedRegions(self.raw_mut(), regions.len() as _, regions.as_ptr())
        }
    }

    pub fn build<Op: OperationExt>(mut self) -> Op {
        unsafe { Op::from_raw(mlirOperationCreate(self.raw_mut())) }
    }
}

/// Helper function to feed the output of an MLIR `*Print()` function into an
/// `std::fmt::Formatter`.
unsafe extern "C" fn print_formatter_callback(string: MlirStringRef, to: *mut std::ffi::c_void) {
    let to: &mut std::fmt::Formatter = std::mem::transmute(to);
    to.write_str(
        std::str::from_utf8(std::slice::from_raw_parts(
            string.data as *const _,
            string.length as usize,
        ))
        .expect("utf8 string"),
    )
    .unwrap();
}
