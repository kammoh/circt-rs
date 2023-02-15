#[macro_use]
pub(crate) mod macros;

pub mod builtin;
pub mod cf;
pub mod comb;
pub mod error;
pub mod esi;
pub mod firrtl;
pub mod fsm;
pub mod func;
pub mod hw;
pub mod mlir;
pub mod seq;
pub mod sv;
pub mod wrap_raw;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::*;
    pub use mlir::*;
}

mod crate_prelude {
    pub use crate::mlir::*;
    pub use crate::prelude::*;
    pub(crate) use crate::wrap_raw::{HasRaw, ToRawVec, WrapRaw, WrapRawPtr};
}

pub(crate) use crate_prelude::*;

use circt_sys::*;

pub fn register_circt_passes() {
    unsafe { circtRegisterTransformsPasses() }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::path::Path;

    #[test]
    fn test_module_build() -> miette::Result<()> {
        let ctx = OwnedContext::default();
        ctx.attach_diagnostic_handler(Box::new(PrintHandler::default()));

        comb::dialect().load(&ctx).unwrap();
        seq::dialect().load(&ctx).unwrap();
        hw::dialect().load(&ctx).unwrap();
        mlir::register_cse();
        mlir::register_canonicalize();
        hw::register_passes();
        hw::register_arith_passes();
        seq::register_passes();
        fsm::register_passes();
        sv::register_passes();

        let mut builder = OpBuilder::new(&ctx);
        let module = Module::create(builder.loc());

        let hw_module_name = "test_hw_module";
        let mut ports = hw::ModulePortInfo::default();

        let i1 = IntegerType::new(&ctx, 1);
        let i2 = IntegerType::new(&ctx, 2);

        ports.add_input("a", &i2);
        ports.add_input("b", &i2);
        ports.add_input("clk", &i1);
        ports.add_output("c", &i2);
        ports.add_output("c1", &i1);

        hw::HwModuleOp::build_with(
            &mut builder,
            &module,
            hw_module_name,
            &ports,
            &[],
            "no comments!",
            |builder, _, inputs, outputs| {
                let c1 = hw::ConstantOp::build(builder, 1, 1).result();
                outputs.insert("c1".to_string(), c1);

                let a_and_b =
                    comb::AndOp::build(builder, &[inputs["a"], inputs["b"]]).unwrap().result();
                let c_reg =
                    seq::CompRegOp::build(builder, "c_reg", &a_and_b, &inputs["clk"], None, None)
                        .unwrap()
                        .result();
                outputs.insert("c".to_string(), c_reg);
            },
        )?;

        // let pm = OwnedPassManager::new(&ctx);
        let pm = PassManager::new(&ctx);
        pm.enable_verifier(true);

        #[rustfmt::skip]
        pm
            .parse_pass("lower-hwarith-to-hw")?
            .nest("hw.module")
                .parse_pass("lower-seq-hlmem")?;
        #[rustfmt::skip]
        pm
            .parse_pass("convert-fsm-to-sv")?
            .parse_pass("cse, canonicalize, cse")?
            // Print a DOT graph of the HWModule's within a top-level module.
            .parse_pass("hw-print-module-graph")?
            .parse_pass("lower-seq-to-sv")?
            .nest("hw.module")
                .parse_pass("cse")?
                .parse_pass("canonicalize")?
                .parse_pass("hw-cleanup")?
                .parse_pass("prettify-verilog")?
                .parse_pass("hw-cleanup")?;
        #[rustfmt::skip]
        pm
            // Print a DOT graph of the module hierarchy.
            .parse_pass("hw-print-instance-graph")?
            .run(&module)?;

        let out_dir = Path::new(hw_module_name);
        std::fs::create_dir_all(out_dir).unwrap();
        sv::export_split_verilog(&module, &out_dir);

        Ok(())
    }
}
