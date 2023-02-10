// Copyright (c) 2022-2023 Kamyar Mohajerani

use circt_sys::registerFirrtlPasses;
define_dialect!(firrtl);

pub fn register_firrtl_passes() {
    unsafe { registerFirrtlPasses() }
}