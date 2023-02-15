// Copyright (c) 2016-2021 Fabian Schuiki

//! Utilities to deal with the MLIR context.

use crate::crate_prelude::*;
use circt_sys::*;
use num_derive::FromPrimitive;

use super::string::StringRef;

wrap_raw_ptr!(Diagnostic => MlirDiagnostic);

#[derive(Debug, FromPrimitive)]
pub enum DiagnosticSeverity {
    Error = MlirDiagnosticSeverity_MlirDiagnosticError as _,
    Warning = MlirDiagnosticSeverity_MlirDiagnosticWarning as _,
    Note = MlirDiagnosticSeverity_MlirDiagnosticNote as _,
    Remark = MlirDiagnosticSeverity_MlirDiagnosticRemark as _,
}

impl Diagnostic {
    pub fn location(&self) -> Location {
        Location::try_from_raw(unsafe { mlirDiagnosticGetLocation(self.0) }).unwrap()
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        num::FromPrimitive::from_u32(unsafe { mlirDiagnosticGetSeverity(self.0) }).unwrap()
    }

    pub fn num_notes(&self) -> usize {
        unsafe { mlirDiagnosticGetNumNotes(self.0) as _ }
    }

    pub fn get_note(&self, pos: usize) -> Diagnostic {
        Diagnostic::try_from_raw(unsafe { mlirDiagnosticGetNote(self.0, pos as _) as _ }).unwrap()
    }
}

wrap_raw_ptr!(Context);

/// An owned MLIR context.
// pub type OwnedContext = Owned<Context>;

impl Context {
    /// Get an interned identifier.
    pub fn get_identifier(&self, ident: &str) -> MlirIdentifier {
        let string_ref = StringRef::from_str(ident);
        unsafe { mlirIdentifierGet(self.raw(), string_ref.raw()) }
    }

    /// Create a new integer type of a given width.
    pub fn get_integer_type(&self, bitwidth: u32) -> Option<Type> {
        Type::try_from_raw(unsafe { mlirIntegerTypeGet(self.raw(), bitwidth) })
    }

    /// Create a new function type.
    pub fn get_function_type(
        &self,
        inputs: impl Iterator<Item = impl Ty>,
        results: impl Iterator<Item = impl Ty>,
    ) -> Option<Type> {
        let inputs: Vec<MlirType> = inputs.into_iter().map(|x| x.raw()).collect();
        let results: Vec<MlirType> = results.into_iter().map(|x| x.raw()).collect();
        Type::try_from_raw(unsafe {
            mlirFunctionTypeGet(
                self.raw(),
                inputs.len() as _,
                inputs.as_ptr(),
                results.len() as _,
                results.as_ptr(),
            )
        })
    }

    /// Create a new struct type.
    pub fn get_struct_type(
        &self,
        fields: impl IntoIterator<Item = (impl AsRef<str>, Type)>,
    ) -> Option<Type> {
        let raw_fields: Vec<_> = fields
            .into_iter()
            .map(|(ident, ty)| HWStructFieldInfo {
                name: self.get_identifier(ident.as_ref()),
                type_: ty.raw(),
            })
            .collect();
        Type::try_from_raw(unsafe {
            hwStructTypeGet(self.raw(), raw_fields.len() as _, raw_fields.as_ptr())
        })
    }

    /// Change whether this MLIR context allows unregistered dialect ops.
    pub fn set_allow_unregistered_dialects(&self, allow: bool) {
        unsafe { mlirContextSetAllowUnregisteredDialects(self.raw(), allow) }
    }

    /// Change whether this MLIR context allows unregistered dialect ops.
    pub fn are_unregistered_dialects_allowed(&self) -> bool {
        unsafe { mlirContextGetAllowUnregisteredDialects(self.raw()) }
    }
    /// Number of loaded dialects. The built-in dialect is always loaded.
    pub fn num_loaded_dialects(&self) -> usize {
        unsafe { mlirContextGetNumLoadedDialects(self.raw()) as _ }
    }

    /// Attaches the diagnostic handler to the context.
    /// Handlers are invoked in the reverse order of attachment until one of them processes the diagnostic completely.
    /// When a handler is invoked it is passed the userData that was provided when it was attached.
    /// If non-NULL, `drop_user_data` is called once the system no longer needs to call the handler
    ///  (for instance after the handler is detached or the context is destroyed).
    /// Returns an identifier that can be used to detach the handler.
    pub fn attach_diagnostic_handler<T: HandlerObject>(&self, handler_obj: Box<T>) -> u64 {
        let handler_obj = Box::leak(handler_obj);
        unsafe {
            mlirContextAttachDiagnosticHandler(
                self.raw(),
                Some(diagnostic_handler::<T>),
                handler_obj as *const _ as *mut _,
                Some(drop_handler::<T>),
            )
        }
    }

    /// Detaches an attached diagnostic handler from the context given its identifier.
    pub fn detach_diagnostic_handler(&self, id: u64) {
        unsafe { mlirContextDetachDiagnosticHandler(self.raw(), id) }
    }
}

impl_create!(Context);
impl_into_owned!(Context);

pub trait HandlerObject {
    fn handle(&mut self, diag: Diagnostic) -> LogicalResult;
}

#[derive(Default, Debug)]
pub struct PrintHandler(bool);

impl HandlerObject for PrintHandler {
    fn handle(&mut self, diag: Diagnostic) -> LogicalResult {
        println!("Severity: {:?}", diag.severity());
        self.0.into()
    }
}

unsafe extern "C" fn diagnostic_handler<T: HandlerObject>(
    diag: MlirDiagnostic,
    use_data: *mut std::ffi::c_void,
) -> MlirLogicalResult {
    let mut user_data = unsafe { Box::from_raw(use_data as *mut T) };
    let res = user_data.handle(Diagnostic::from_raw(diag)).raw();
    Box::leak(user_data);
    res
}
unsafe extern "C" fn drop_handler<T: HandlerObject>(use_data: *mut std::ffi::c_void) {
    if !use_data.is_null() {
        let user_data = unsafe { Box::from_raw(use_data as *mut T) };
        drop(user_data)
    }
}

impl Default for Owned<Context> {
    fn default() -> Self {
        Self(Context::create())
    }
}
