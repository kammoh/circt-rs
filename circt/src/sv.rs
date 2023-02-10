// Copyright (c) 2022-2023 Kamyar Mohajerani

use circt_sys::registerSVPasses;

define_dialect!(sv);

pub fn register_sv_passes() {
    unsafe { registerSVPasses() }
}
