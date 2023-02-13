// Copyright (c) 2022-2023 Kamyar Mohajerani

use circt_sys::registerFSMPasses;

define_dialect!(fsm);

pub fn register_passes() {
    unsafe { registerFSMPasses() }
}
