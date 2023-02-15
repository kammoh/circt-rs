# circt
[CIRCT](https://github.com/llvm/circt) Rust bindings. 

It's originally based on [moore's](https://github.com/fabianschuiki/moore) CIRCT rust wrapper by Fabian Schuiki.

## Example
```rust
let ctx = OwnedContext::default();
comb::dialect().load(&ctx).unwrap();
seq::dialect().load(&ctx).unwrap();
hw::dialect().load(&ctx).unwrap();
mlir::register_passes();
hw::register_passes();
seq::register_passes();
sv::register_passes();

let mut builder = OpBuilder::new(&ctx);
let module = Module::create(builder.loc());

let hw_module_name = "test_hw_module";
let mut ports = ModulePortInfo::default();

let i1 = IntegerType::new(&ctx, 1);
let i2 = IntegerType::new(&ctx, 2);

ports.add_input("a", &i2);
ports.add_input("b", &i2);
ports.add_input("clk", &i1);
ports.add_output("c", &i2);

hw::HwModuleOp::build_with(
    &mut builder,
    &module,
    hw_module_name,
    &ports,
    &[],
    "no comments!",
    |builder, _, inputs, outputs| {
        let a_and_b =
            comb::AndOp::build(builder, &[inputs["a"], inputs["b"]]).unwrap().result();
        let c_reg =
            CompRegOp::build(builder, "c_reg", &a_and_b, &inputs["clk"], None, None)
                .unwrap()
                .result();
        outputs.insert("c".to_string(), c_reg);
    },
)
.unwrap();

let pm = OwnedPassManager::new(&ctx);
pm.parse_pass("cse, canonicalize, cse").unwrap()
    // Print a DOT graph of the HWModule's within a top-level module.
    .parse_pass("hw-print-module-graph").unwrap()
    .parse_pass("lower-seq-to-sv").unwrap()
    .nest("hw.module")
        .parse_pass("hw-cleanup").unwrap()
        .parse_pass("prettify-verilog").unwrap()
        .run(&module).unwrap();

// generate SystemVerilog in `out_dir`
let out_dir = Path::new(hw_module_name);
std::fs::create_dir_all(out_dir).unwrap();
sv::export_split_verilog(&module, &out_dir);
```

#### License
Licensed under either of <a href="License-Apache.md">Apache License, Version 2.0</a> or <a href="License-MIT.md">MIT license</a> at your option.
