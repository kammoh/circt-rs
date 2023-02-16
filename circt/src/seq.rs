// Copyright (c) 2022-2023 Kamyar Mohajerani
use crate::crate_prelude::*;
use circt_sys::*;

// Types and operations for seq dialect
// The seq dialect is intended to model digital sequential logic.

define_dialect!(seq);

pub fn register_passes() {
    unsafe { circt_sys::registerSeqPasses() }
}

pub fn create_seq_lower_to_sv_pass() -> Pass {
    Pass::try_from_raw(unsafe { seqCreateSeqLowerToSVPass() }).unwrap()
}

def_operation_single_result!(CompRegClockEnabledOp, "seq.compreg.ce");

impl CompRegClockEnabledOp {
    pub fn input(&self) -> Value {
        self.operand(0).unwrap()
    }

    pub fn set_input(&self, new_value: &Value) {
        self.set_operand(0, new_value);
    }

    pub fn clk(&self) -> Value {
        self.operand(1).unwrap()
    }

    pub fn set_clk(&self, new_value: &Value) {
        self.set_operand(1, new_value);
    }

    pub fn clock_enable(&self) -> Value {
        self.operand(2).unwrap()
    }

    pub fn set_clock_enable(&self, new_value: &Value) {
        self.set_operand(2, new_value);
    }
    pub fn reset(&self) -> Value {
        self.operand(3).unwrap()
    }

    pub fn set_reset(&self, new_value: &Value) {
        self.set_operand(3, new_value);
    }

    pub fn reset_value(&self) -> Value {
        self.operand(4).unwrap()
    }

    pub fn set_reset_value(&self, new_value: &Value) {
        self.set_operand(4, new_value);
    }

    pub fn output(&self) -> Value {
        self.result()
    }

    pub fn build(
        builder: &mut OpBuilder,
        name: &str,
        input: &Value,
        clk: &Value,
        clock_enable: &Value,
        reset: Option<&Value>,
        reset_value: Option<&Value>,
    ) -> Option<Self> {
        builder.build_with(|builder, state| {
            let ctx = builder.context();
            state.add_attribute("name", &StringAttr::new(&ctx, name));
            state.add_operand(input);
            state.add_operand(clk);
            state.add_operand(clock_enable);
            if let Some(reset) = reset {
                state.add_operand(reset);
                if let Some(reset_value) = reset_value {
                    state.add_operand(reset_value);
                }
            } else {
                assert!(reset_value.is_none());
            }
            state.add_result(&input.ty());
        })
    }
}

def_operation_single_result!(CompRegOp, "seq.compreg");

impl CompRegOp {
    pub fn input(&self) -> Value {
        self.operand(0).unwrap()
    }

    pub fn set_input(&self, new_value: &Value) {
        self.set_operand(0, new_value);
    }

    pub fn clk(&self) -> Value {
        self.operand(1).unwrap()
    }

    pub fn set_clk(&self, new_value: &Value) {
        self.set_operand(1, new_value);
    }

    pub fn reset(&self) -> Value {
        self.operand(2).unwrap()
    }

    pub fn set_reset(&self, new_value: &Value) {
        self.set_operand(2, new_value);
    }

    pub fn reset_value(&self) -> Value {
        self.operand(3).unwrap()
    }

    pub fn set_reset_value(&self, new_value: &Value) {
        self.set_operand(3, new_value);
    }

    pub fn output(&self) -> Value {
        self.result()
    }

    pub fn build(
        builder: &mut OpBuilder,
        name: &str,
        input: &Value,
        clk: &Value,
        reset: Option<&Value>,
        reset_value: Option<&Value>,
    ) -> Option<Self> {
        builder.build_with(|builder, state| {
            let ctx = builder.context();
            state.add_attribute("name", &StringAttr::new(&ctx, name));
            state.add_operand(input);
            state.add_operand(clk);
            if let Some(reset) = reset {
                state.add_operand(reset);
                if let Some(reset_value) = reset_value {
                    state.add_operand(reset_value);
                }
            } else {
                assert!(reset_value.is_none());
            }
            state.add_result(&input.ty());
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{crate_prelude::*, hw::ConstantOp};

    use super::CompRegOp;

    #[test]
    fn test_comp_reg() {
        let ctx = OwnedContext::default();
        let mut builder = OpBuilder::new(&ctx);

        hw::dialect().load(&ctx);
        seq::dialect().load(&ctx);

        let region = Region::default();
        let block = Block::default();
        region.append_block(&block);

        builder.set_insertion_point(Some(InsertPoint::BlockEnd(block.clone())));

        let input = ConstantOp::build(&mut builder, 8, 123).result();
        let clk = ConstantOp::build(&mut builder, 1, 1).result();

        let reg = CompRegOp::build(&mut builder, "reg1", &input, &clk, None, None).unwrap();

        assert_eq!(reg.verify(), true);

        println!("reg> {:?}", reg);
        println!("block> {:?}", block);
    }
}
