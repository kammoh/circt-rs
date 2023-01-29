// Copyright (c) 2016-2021 Fabian Schuiki

use circt_sys::capi::mlirGetDialectHandle__seq__;

use crate::mlir::DialectHandle;

pub fn dialect() -> DialectHandle {
    DialectHandle::from_raw(unsafe { mlirGetDialectHandle__seq__() })
}
