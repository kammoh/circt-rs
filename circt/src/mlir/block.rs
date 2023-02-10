// Copyright (c) 2022-2023 Kamyar Mohajerani

use std::fmt::Debug;

use crate::crate_prelude::*;
use circt_sys::*;

wrap_raw_ptr!(Block, Clone);
impl_into_owned!(Block);

impl Block {
    /// Creates a new empty block with the given argument types and transfers ownership to the caller.
    pub fn create(args_and_locs: &[(Type, Location)]) -> Option<Self> {
        let (args, locs): (Vec<_>, Vec<_>) = args_and_locs
            .iter()
            .map(|(t, l)| (t.raw(), l.raw()))
            .unzip();
        Self::try_from_raw(unsafe {
            mlirBlockCreate(args.len() as _, args.as_ptr(), locs.as_ptr())
        })
    }

    pub fn new() -> Self {
        Self::create(&[]).unwrap()
    }

    /// Returns the region that contains this block.
    pub fn parent_region(&self) -> Option<Region> {
        Region::try_from_raw(unsafe { mlirBlockGetParentRegion(self.raw()) })
    }

    /// Returns the closest surrounding operation that contains this block.
    pub fn parent_operation(&self) -> Option<Operation> {
        Operation::try_from_raw(unsafe { mlirBlockGetParentOperation(self.raw()) })
    }

    /// Returns the terminator operation in the block or `None` if no terminator.
    pub fn terminator(&self) -> Option<Operation> {
        Operation::try_from_raw(unsafe { mlirBlockGetTerminator(self.raw()) })
    }

    /// Detach a block from the owning region and assume ownership.
    pub fn detach(self) {
        unsafe { mlirBlockDetach(self.raw()) }
    }

    /// Get the number of arguments.
    pub fn num_arguments(&self) -> usize {
        unsafe { mlirBlockGetNumArguments(self.raw()) as _ }
    }

    /// Get an argument by index.
    pub fn argument(&self, index: usize) -> Option<Value> {
        Value::try_from_raw(unsafe { mlirBlockGetArgument(self.raw(), index as _) })
    }

    /// Get an iterator over all arguments.
    pub fn arguments(&self) -> impl Iterator<Item = Value> + '_ {
        Box::new((0..self.num_arguments()).map(move |i| self.argument(i).unwrap()))
    }

    /// Appends an argument of the specified type to the block. Returns the newly added argument.
    pub fn add_argument(&self, ty: &impl Ty, loc: &Location) -> Option<Value> {
        Value::try_from_raw(unsafe { mlirBlockAddArgument(self.raw(), ty.raw(), loc.raw()) })
    }

    /// Takes an operation owned by the caller and appends it to the block.
    pub fn append_op(&self, op: &impl HasRaw<RawType = MlirOperation>) {
        unsafe { mlirBlockAppendOwnedOperation(self.raw(), op.raw()) };
    }

    /// Takes an operation owned by the caller and inserts it after the (non-owned) reference operation in the given block.
    /// If the reference is null, prepends the operation. Otherwise, the reference must belong to the block.
    pub fn insert_op_after(
        &self,
        op: &impl HasRaw<RawType = MlirOperation>,
        ref_op: &impl HasRaw<RawType = MlirOperation>,
    ) {
        unsafe { mlirBlockInsertOwnedOperationAfter(self.raw(), ref_op.raw(), op.raw()) }
    }

    /// Prepend an operation owned by the caller to the block
    pub fn prepend_op(&self, op: &impl HasRaw<RawType = MlirOperation>) {
        unsafe {
            mlirBlockInsertOwnedOperationAfter(self.raw(), Operation::null_op().raw(), op.raw())
        }
    }

    /// Takes an operation owned by the caller and inserts it before the (non-owned) reference operation in the given block.
    /// If the reference is null, appends the operation. Otherwise, the reference must belong to the block.
    pub fn insert_op_before(
        &self,
        op: &impl HasRaw<RawType = MlirOperation>,
        ref_op: &impl HasRaw<RawType = MlirOperation>,
    ) {
        unsafe { mlirBlockInsertOwnedOperationBefore(self.raw(), ref_op.raw(), op.raw()) }
    }

    /// Takes an operation owned by the caller and inserts it as pos to the block.
    /// This is an expensive operation that scans the block linearly, prefer insert_op_before/after and append_op/prepend_op instead.
    pub fn insert_op_at(&self, pos: usize, op: &impl HasRaw<RawType = MlirOperation>) {
        unsafe { mlirBlockInsertOwnedOperation(self.raw(), pos.try_into().unwrap(), op.raw()) }
    }

    /// Print the block to anything that implements `std::io::Write`.
    pub fn print<T: std::fmt::Write + Sized>(&self, w: &mut T) -> std::fmt::Result {
        let fmt = FormatterCallback::new(w);
        // Prints a block by sending chunks of the string representation and forwarding userData to callback`.
        // Note that the callback may be called several times with consecutive chunks of the string.
        unsafe { mlirBlockPrint(self.raw(), fmt.callback(), fmt.user_data()) };
        Ok(())
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirBlockEqual(self.0, other.0) }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}

wrap_raw_ptr!(Region);
impl_into_owned!(Region);

impl Region {
    /// Creates a new empty region and transfers ownership to the caller.
    pub fn new() -> Self {
        Self::try_from_raw(unsafe { mlirRegionCreate() }).unwrap()
    }

    /// Takes a block owned by the caller and inserts it after the (non-owned) reference block in this region.
    /// The reference block must belong to this region.
    pub fn insert_block_after(&self, reference: &Block, block: &Block) {
        unsafe {
            // If the reference block is null, prepends the block to the region.
            mlirRegionInsertOwnedBlockAfter(self.raw(), reference.raw(), block.raw());
        }
    }

    /// Takes a block owned by the caller and inserts it before the (non-owned) reference block in this region.
    /// The reference block must belong to this region.
    pub fn insert_block_before(&self, reference: &Block, block: &Block) {
        unsafe {
            // If the reference block is null, appends the block to the region.
            mlirRegionInsertOwnedBlockBefore(self.raw(), reference.raw(), block.raw());
        }
    }

    /// Takes a block owned by the caller and inserts it at pos to the given region.
    /// This is an expensive operation that linearly scans the region, prefer insert_block_before/after instead.
    pub fn insert_block(&self, pos: usize, block: &Block) {
        unsafe { mlirRegionInsertOwnedBlock(self.raw(), pos.try_into().unwrap(), block.raw()) }
    }

    /// Takes a block owned by the caller and prepends it to the region.
    pub fn prepend_block(&self, block: &Block) {
        unsafe {
            // If the reference block is null, prepends the block to the region.
            mlirRegionInsertOwnedBlockAfter(self.raw(), MlirBlock { ptr: 0 as _ }, block.raw());
        }
    }

    /// Takes a block owned by the caller and appends it to this region.
    pub fn append_block(&self, block: &Block) {
        unsafe {
            // If the reference block is null, prepends the block to the region.
            mlirRegionAppendOwnedBlock(self.raw(), block.raw());
        }
    }

    /// Gets the first block in the region.
    pub fn first_block(&self) -> Option<Block> {
        Block::try_from_raw(unsafe { mlirRegionGetFirstBlock(self.raw()) })
    }

    /// Returns the region immediately following the given region in its parent operation.
    pub fn next(&self) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirRegionGetNextInOperation(self.raw()) })
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirRegionEqual(self.0, other.0) }
    }
}
