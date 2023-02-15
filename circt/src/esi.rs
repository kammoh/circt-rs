use circt_sys::*;

define_dialect!(esi);

pub fn register_passes() {
    unsafe { registerESIPasses() }
}

pub fn register_translations() {
    unsafe { registerESITranslations() }
}
