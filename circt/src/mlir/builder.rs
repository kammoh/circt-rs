// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

//! A builder for IR operations.

use crate::crate_prelude::*;

/// A builder for MLIR operations.
pub struct OpBuilder<'a> {
    /// The surrounding MLIR context.
    ctx: &'a Context,
    /// The location to assign to the operations being built.
    loc: Location,
    pub insert_point: Option<InsertPoint>,
}

impl<'a> OpBuilder<'a> {
    /// Create a new builder.
    pub fn new(ctx: &'a Context) -> Self {
        Self::new_with_loc(ctx, Location::new_unknown(ctx))
    }

    /// Create a new builder with Location loc
    pub fn new_with_loc(ctx: &'a Context, loc: Location) -> Self {
        Self {
            ctx,
            loc,
            insert_point: None,
        }
    }

    pub fn context(&self) -> &Context {
        self.ctx
    }

    /// Set the location assigned to new operations.
    pub fn set_loc(&mut self, loc: Location) {
        self.loc = loc;
    }

    /// Get the current location that is assigned to new operations.
    pub fn loc(&self) -> &Location {
        &self.loc
    }

    pub fn set_insertion_point(&mut self, insert_point: Option<InsertPoint>) {
        self.insert_point = insert_point;
    }

    pub fn insert_in(&self, op: &impl Op, insert_point: &InsertPoint) {
        match insert_point {
            InsertPoint::BlockStart(block) => block.prepend_op(op),
            InsertPoint::BlockEnd(block) => block.append_op(op),
            InsertPoint::AfterOp(block, ref_op) => block.insert_op_after(op, ref_op),
            InsertPoint::BeforeOp(block, ref_op) => block.insert_op_before(op, ref_op),
        }
    }

    pub fn insert(&self, op: &impl Op) {
        if let Some(ref insert_point) = self.insert_point {
            self.insert_in(op, insert_point)
        }
    }

    /// Build an operation through a callback that populates an
    /// `OperationState`.
    pub fn build_with<Op: NamedOp>(
        &mut self,
        with_fn: impl FnOnce(&mut Self, &mut OperationState),
    ) -> Option<Op> {
        let mut state = OperationState::new(Op::operation_name(), &self.loc);
        with_fn(self, &mut state);
        let op: Op = state.build()?;
        self.insert(&op);
        Some(op)
    }

    pub fn build_with_failable<Op: NamedOp>(
        &mut self,
        with_fn: impl FnOnce(&mut Self, &mut OperationState) -> Result<(), ()>,
    ) -> Option<Op> {
        let mut state = OperationState::new(Op::operation_name(), &self.loc);
        with_fn(self, &mut state).ok()?;
        let op: Op = state.build()?;
        self.insert(&op);
        Some(op)
    }
    //// Create a new block after the current one.
    // pub fn add_block(&mut self) -> Block {
    //     let ref_block = self.insert_block.as_ref().expect("insertion block not set");
    //     let new_block = Block::create(&[], &[]).unwrap();
    //     let region = ref_block.get_parent_region().unwrap();
    //     // let after = self
    //     //     .insert_block_after
    //     //     .as_ref()
    //     //     .expect("insertion block not set");
    //     // region.insert_block_after(after, &new_block);
    //     // let new_block = new_block;
    //     // self.insert_block_after = Some(new_block.clone());
    //     // FIXME!!! multiple copies of MlirBlock floating around, not safe to _Destroy
    //     new_block
    // }

    // pub fn unknown_location(&self) -> Location {
    //     Location::unknown(self.ctx)
    // }
}

// #[derive(Clone, Copy, Default)]
// pub enum InsertPoint<'a, 'b> {
//     BlockStart(&'a Block),
//     BlockEnd(&'a Block),
//     AfterOp(&'a Block, &'b Operation),
//     BeforeOp(&'a Block, &'b Operation),
//     #[default]
//     None,
// }

#[derive(Clone)]
pub enum InsertPoint {
    BlockStart(Block),
    BlockEnd(Block),
    AfterOp(Block, Operation),
    BeforeOp(Block, Operation),
}
