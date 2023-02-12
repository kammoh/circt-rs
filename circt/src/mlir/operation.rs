use std::borrow::Borrow;

use crate::crate_prelude::*;
use circt_sys::*;

use self::string::StringRef;

wrap_raw_ptr!(OpPrintingFlags);
impl_create!(OpPrintingFlags);
impl_into_owned!(OpPrintingFlags);

wrap_raw_ptr!(Operation, Clone, Copy);
impl_into_owned!(Operation);

impl Operation {
    #[inline(always)]
    pub fn null_op() -> Self {
        Operation(MlirOperation {
            ptr: std::ptr::null_mut(),
        })
    }

    pub fn try_into_op<T: NamedOp>(self) -> Option<T> {
        T::isa(&self).then_some(T::try_from_raw(self.raw())?)
    }
}

pub trait Op: WrapRawPtr<RawType = MlirOperation> {
    /// Gets the name of the operation as an identifier.
    fn name(&self) -> Identifier {
        Identifier::try_from_raw(unsafe { mlirOperationGetName(self.raw()) }).unwrap()
    }

    /// Creates an operation and transfers ownership to the caller.
    ///
    /// **Note that caller owned child objects are transferred in this call and must not be further used.**
    /// Particularly, this applies to any **regions** added to the state (the implementation may invalidate any such pointers).
    ///
    /// This call can fail under the following conditions, in which case, it will return a null operation and emit diagnostics:
    ///  - Result type inference is enabled and cannot be performed.
    fn create(state: &mut OperationState) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirOperationCreate(state.raw_mut()) })
    }

    /// Returns pos-th region attached to the operation.
    fn region(&self, pos: usize) -> Option<Region> {
        Region::try_from_raw(unsafe { mlirOperationGetRegion(self.raw(), pos.try_into().unwrap()) })
    }

    /// Returns first region attached to the operation.
    fn first_region(&self) -> Option<Region> {
        Region::try_from_raw(unsafe { mlirOperationGetFirstRegion(self.raw()) })
    }

    /// Returns first block in the first region attached to the operation.
    fn first_block(&self) -> Option<Block> {
        let region = self.first_region()?;
        region.first_block()
    }

    /// Returns an operation immediately following the given operation it its enclosing block.
    fn next(&self) -> Option<Operation> {
        Operation::try_from_raw(unsafe { mlirOperationGetNextInBlock(self.raw()) })
    }

    /// Returns the number of operands of the operation.
    fn num_operands(&self) -> isize {
        unsafe { mlirOperationGetNumOperands(self.raw()) }
    }

    /// Returns pos-th operand of the operation.
    fn operand(&self, pos: usize) -> Option<Value> {
        Value::try_from_raw(unsafe { mlirOperationGetOperand(self.raw(), pos.try_into().unwrap()) })
    }

    /// Sets the pos-th operand of the operation.
    fn set_operand(&self, pos: usize, new_value: Value) {
        unsafe { mlirOperationSetOperand(self.raw(), pos.try_into().unwrap(), new_value.raw()) }
    }

    /// Returns the number of results of the operation.
    fn num_results(&self) -> usize {
        unsafe { mlirOperationGetNumResults(self.raw()).try_into().unwrap() }
    }

    /// Returns pos-th result of the operation.
    fn result_at(&self, pos: usize) -> Option<Value> {
        Value::try_from_raw(unsafe { mlirOperationGetResult(self.raw(), pos.try_into().unwrap()) })
    }

    /// Returns the number of successor blocks of the operation.
    fn num_successors(&self) -> usize {
        unsafe {
            mlirOperationGetNumSuccessors(self.raw())
                .try_into()
                .unwrap()
        }
    }

    /// Returns pos-th successor of the operation.
    fn successor(&self, pos: usize) -> Option<Block> {
        Block::try_from_raw(unsafe {
            mlirOperationGetSuccessor(self.raw(), pos.try_into().unwrap())
        })
    }

    /// Returns the number of attributes attached to the operation.
    fn num_attributes(&self) -> usize {
        unsafe {
            mlirOperationGetNumAttributes(self.raw())
                .try_into()
                .unwrap()
        }
    }

    /// Returns pos-th successor of the operation.
    fn attribute(&self, pos: usize) -> Option<NamedAttribute> {
        if pos >= self.num_attributes() {
            return None;
        }
        NamedAttribute::try_from_raw(unsafe {
            mlirOperationGetAttribute(self.raw(), pos.try_into().unwrap())
        })
    }

    /// Returns an attribute attached to the operation given its name.
    fn attribute_by_name(&self, name: &str) -> Option<Attribute> {
        Attribute::try_from_raw(unsafe {
            mlirOperationGetAttributeByName(self.raw(), StringRef::from_str(name).raw())
        })
    }

    /// Sets an attribute by name, replacing the existing if it exists or adding a new one otherwise.
    fn set_attribute_by_name(&self, name: &str, attr: impl Attr) {
        unsafe {
            mlirOperationSetAttributeByName(self.raw(), StringRef::from_str(name).raw(), attr.raw())
        }
    }

    /// Removes an attribute by name.
    /// Returns false if the attribute was not found and true if removed.
    fn remove_attribute_by_name(&self, name: &str) -> bool {
        unsafe { mlirOperationRemoveAttributeByName(self.raw(), StringRef::from_str(name).raw()) }
    }

    /// Print the operation to anything that implements `std::io::Write`.
    fn print<T: std::fmt::Write>(&self, to: &mut T, with_debug_info: bool) {
        let fmt = FormatterCallback::new(to);
        // Print the operation through the above callback, which basically just
        // forwards the chunks to the Rust-native `Write` implementation.

        // Prints an operation by sending chunks of the string representation and forwarding userData to callback`.
        // Note that the callback may be called several times with consecutive chunks of the string.
        let flags = OpPrintingFlags::create().unwrap();
        if with_debug_info {
            unsafe {
                mlirOpPrintingFlagsEnableDebugInfo(flags.raw(), true, true);
                mlirOperationPrintWithFlags(
                    self.raw(),
                    flags.raw(),
                    fmt.callback(),
                    fmt.user_data(),
                )
            }
        } else {
            unsafe { mlirOperationPrint(self.raw(), fmt.callback(), fmt.user_data()) }
        }
    }

    /// Print the operation to stderr.
    fn dump(&self) {
        unsafe { mlirOperationDump(self.raw()) };
    }

    /// Verify the operation and return true if it passes, false if it fails.
    fn verify(&self) -> bool {
        unsafe { mlirOperationVerify(self.raw()) }
    }

    /// Moves the given operation immediately after the other operation in its parent block.
    /// The given operation may be owned by the caller or by its current block.
    /// The other operation must belong to a block.
    /// In any case, the ownership is transferred to the block of the other operation.
    fn move_after(&self, other: impl Op) {
        unsafe { mlirOperationMoveAfter(self.raw(), other.raw()) }
    }

    /// Moves the given operation immediately before the other operation in its parent block.
    /// The given operation may be owner by the caller or by its current block.
    /// The other operation must belong to a block.
    /// In any case, the ownership is transferred to the block of the other operation.
    fn move_before(&self, other: impl Op) {
        unsafe { mlirOperationMoveBefore(self.raw(), other.raw()) }
    }

    /// Gets the context this operation is associated with
    fn context(&self) -> Context {
        Context::try_from_raw(unsafe { mlirOperationGetContext(self.raw()) }).unwrap()
    }

    /// Gets the block that owns this operation, returning None if the operation is not owned.
    fn parent_block(&self) -> Option<Block> {
        Block::try_from_raw(unsafe { mlirOperationGetBlock(self.raw()) })
    }
}

impl<T> Op for T where T: NamedOp {}

impl Op for Operation {}

/// A trait implemented by anything that wraps an MLIR operation.
pub trait NamedOp: WrapRawPtr<RawType = MlirOperation> {
    const OP_NAME: Option<&'static str> = None;

    /// Return the full operation name, like `builtin.module`.
    fn operation_name() -> &'static str {
        Self::OP_NAME.unwrap()
    }

    fn isa(op: &Operation) -> bool {
        op.name().to_string_ref() == StringRef::from_str(Self::operation_name())
    }
}

/// An operation that has a single region.
pub trait SingleRegionOp: NamedOp {}

/// An operation that has a single block in a single region.
pub trait SingleBlockOp: SingleRegionOp {}

wrap_raw!(OperationState);

impl OperationState {
    /// Constructs an operation state from a name and a location.
    pub fn new(name: &str, loc: &Location) -> Self {
        Self(unsafe { mlirOperationStateGet(StringRef::from_str(name).raw(), loc.raw()) })
    }

    #[inline(always)]
    fn regions_raw(&self) -> &[MlirRegion] {
        unsafe { std::slice::from_raw_parts(self.0.regions, self.0.nRegions as _) }
    }

    pub fn region(&self, idx: usize) -> Option<Region> {
        let regions = self.regions_raw();
        if idx >= regions.len() {
            return None;
        }
        Region::try_from_raw(regions[idx])
    }

    /// Enables result type inference for the operation under construction.
    /// If enabled, then the caller must **not** have called mlirOperationStateAddResults().
    /// Note that if enabled, `self.build()` is failable:
    /// - it will return a `None` on inference failure and will emit diagnostics.
    pub fn enable_result_type_inference(&mut self) {
        unsafe { mlirOperationStateEnableResultTypeInference(self.raw_mut()) }
    }

    /// Add a result to the operation.
    pub fn add_result(&mut self, ty: &impl Ty) {
        self.add_results_raw(&[ty.raw()])
    }

    /// Add multiple results to the operation.
    pub fn add_results_raw(&mut self, types: &[MlirType]) {
        unsafe { mlirOperationStateAddResults(self.raw_mut(), types.len() as _, types.as_ptr()) }
    }

    /// Add multiple results to the operation.
    pub fn add_results(&mut self, types: impl IntoIterator<Item = impl Ty>) {
        let types = types.to_raw_vec();
        self.add_results_raw(types.as_slice())
    }

    /// Add an operand to the operation.
    pub fn add_operand(&mut self, value: &impl HasRaw<RawType = MlirValue>) {
        self.add_operands_raw(&[value.raw()])
    }

    /// Add multiple operands to the operation.
    pub fn add_operands_raw(&mut self, values: &[MlirValue]) {
        unsafe { mlirOperationStateAddOperands(self.raw_mut(), values.len() as _, values.as_ptr()) }
    }

    /// Add multiple operands to the operation.
    pub fn add_operands<V: HasRaw<RawType = MlirValue>>(
        &mut self,
        values: impl IntoIterator<Item = impl Borrow<V>>,
    ) {
        let values: Vec<MlirValue> = values.to_raw_vec();
        self.add_operands_raw(values.as_slice())
    }

    /// Add an attribute to the operation.
    pub fn add_attribute(&mut self, name: &str, attr: &impl Attr) {
        self.add_attributes_raw(&[attr.to_named(name).raw()]);
    }

    /// Add multiple attributes to the operation.
    pub fn add_attributes(&mut self, attrs: impl IntoIterator<Item = impl Borrow<NamedAttribute>>) {
        let attrs: Vec<_> = attrs.to_raw_vec();
        self.add_attributes_raw(attrs.as_slice())
    }

    /// Add multiple attributes to the operation.
    pub fn add_attributes_raw(&mut self, attrs: &[MlirNamedAttribute]) {
        unsafe { mlirOperationStateAddAttributes(self.raw_mut(), attrs.len() as _, attrs.as_ptr()) }
    }

    /// Add a successor to the operation.
    pub fn add_successor(&mut self, block: &Block) {
        self.add_successors_raw(&[block.raw()]);
    }

    /// Add multiple successors to the operation.
    pub fn add_successors_raw(&mut self, blocks: &[MlirBlock]) {
        unsafe {
            mlirOperationStateAddSuccessors(self.raw_mut(), blocks.len() as _, blocks.as_ptr())
        }
    }

    /// Add a region to the operation.
    pub fn add_region(&mut self, region: &Region) {
        self.add_regions_raw(&[region.raw()])
    }

    /// Add multiple regions to the operation.
    pub fn add_regions_raw(&mut self, regions: &[MlirRegion]) {
        unsafe {
            mlirOperationStateAddOwnedRegions(self.raw_mut(), regions.len() as _, regions.as_ptr())
        }
    }
    /// Add multiple regions to the operation.
    pub fn add_regions(&mut self, regions: impl IntoIterator<Item = impl Borrow<Region>>) {
        self.add_regions_raw(&regions.to_raw_vec())
    }

    pub fn build<T: NamedOp>(&mut self) -> Option<T> {
        T::create(self)
    }
}

wrap_raw_ptr!(OpOperand);

impl OpOperand {
    /// Returns whether the op operand is null.
    pub fn is_null(&self) -> bool {
        unsafe { mlirOpOperandIsNull(self.raw()) }
    }

    /// Returns the owner operation of an op operand.
    pub fn owner(&self) -> Option<Operation> {
        Operation::try_from_raw(unsafe { mlirOpOperandGetOwner(self.raw()) })
    }

    /// Returns the operand number of an op operand.
    pub fn operand_number(&self) -> u32 {
        unsafe { mlirOpOperandGetOperandNumber(self.raw()) }
    }
    /// Returns an op operand representing the next use of the value, or a null op operand if there is no next use.
    pub fn next_use(&self) -> Self {
        Self::from_raw(unsafe { mlirOpOperandGetNextUse(self.raw()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hw;

    #[test]
    fn test_print_op() {
        let ctx = OwnedContext::new();
        let loc = Location::new_unknown(&ctx);
        let mut state = OperationState::new(hw::ConstantOp::operation_name(), &loc);
        let op: hw::ConstantOp = state.build().unwrap();
        assert_eq!(op.to_string(), "\"hw.constant\"() : () -> ()\n");
        println!("{:?}", op);
    }
}