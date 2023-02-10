use crate::crate_prelude::*;
// Copyright (c) 2022-2023 Kamyar Mohajerani

define_dialect!(seq);

pub fn register_seq_passes (){
    unsafe{
        circt_sys::registerSeqPasses()
    }
}