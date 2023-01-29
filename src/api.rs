// use super::capi::*;

// // TODO use macros for many of the definitions
// pub trait CPtrIsNull {
//     fn is_null(&self) -> bool;
// }

// impl CPtrIsNull for MlirType {
//     fn is_null(&self) -> bool {
//         self.ptr.is_null()
//     }
// }

// impl CPtrIsNull for MlirDialect {
//     fn is_null(&self) -> bool {
//         self.ptr.is_null()
//     }
// }

// impl CPtrIsNull for MlirModule {
//     fn is_null(&self) -> bool {
//         self.ptr.is_null()
//     }
// }
// impl CPtrIsNull for MlirOperation {
//     fn is_null(&self) -> bool {
//         self.ptr.is_null()
//     }
// }

// impl<T: AsRef<str>> From<T> for MlirStringRef {
//     fn from(s: T) -> Self {
//         let bytes = s.as_ref().as_bytes();
//         MlirStringRef {
//             data: bytes.as_ptr() as _,
//             length: bytes.len() as _,
//         }
//     }
// }

// // impl AsRef<str> for MlirStringRef{
// //     fn as_ref(&self) -> &str {
// //         unsafe {
// //             std::str::from_utf8_unchecked(std::slice::from_raw_parts(
// //                 self.data as _,
// //                 self.length as _,
// //             ))
// //         }
// //     }
// // }

// impl From<MlirStringRef> for &str {
//     fn from(string_ref: MlirStringRef) -> Self {
//         unsafe {
//             std::str::from_utf8_unchecked(std::slice::from_raw_parts(
//                 string_ref.data as _,
//                 string_ref.length as _,
//             ))
//         }
//     }
// }

// pub trait WriterCallBack<T> {
//     fn into_callback(
//         self,
//     ) -> (
//         *mut libc::c_void,
//         extern "C" fn(MlirStringRef, *mut libc::c_void),
//     );
// }

// impl<T: std::io::Write> WriterCallBack<T> for T {
//     fn into_callback(
//         self,
//     ) -> (
//         *mut libc::c_void,
//         extern "C" fn(MlirStringRef, *mut libc::c_void),
//     ) {
//         extern "C" fn callback<W: std::io::Write>(
//             str_ref: MlirStringRef,
//             user_data: *mut ::libc::c_void,
//         ) {
//             let mut b = unsafe { Box::from_raw(user_data as *mut W) };
//             let s: &str = str_ref.into();
//             b.write(s.as_bytes());
//             Box::leak(b);
//         }
//         (Box::into_raw(Box::new(self)) as _, callback::<T>)
//     }
// }

// pub trait PassManagerNest<T> {
//     fn nest(self, op: T) -> MlirOpPassManager; // todo
// }

// impl<T: AsRef<str>> PassManagerNest<T> for MlirPassManager {
//     fn nest(self, op: T) -> MlirOpPassManager {
//         unsafe { mlirPassManagerGetNestedUnder(self, op.into()) }
//     }
// }

// impl<T: AsRef<str>> PassManagerNest<T> for MlirOpPassManager {
//     fn nest(self, op: T) -> MlirOpPassManager {
//         unsafe { mlirOpPassManagerGetNestedUnder(self, op.into()) }
//     }
// }

// pub trait AddPass {
//     fn add_pass(self, pm: MlirPass);
// }

// impl MlirPassManager {
//     pub fn run(self, module: MlirModule) -> MlirLogicalResult {
//         unsafe { mlirPassManagerRun(self, module) }
//     }
//     pub fn create(context: &Context) -> MlirPassManager {
//         unsafe { mlirPassManagerCreate(context.ctx) }
//     }
// }

// impl AddPass for MlirPassManager {
//     fn add_pass(self, pass: MlirPass) {
//         unsafe { mlirPassManagerAddOwnedPass(self, pass) }
//     }
// }

// impl AddPass for MlirOpPassManager {
//     fn add_pass(self, pass: MlirPass) {
//         unsafe { mlirOpPassManagerAddOwnedPass(self, pass) }
//     }
// }

// impl Into<MlirOpPassManager> for MlirPassManager {
//     fn into(self) -> MlirOpPassManager {
//         unsafe { mlirPassManagerGetAsOpPassManager(self) }
//     }
// }

// impl MlirModule {
//     pub fn export_verilog<W: std::io::Write>(self, writer: W) -> MlirLogicalResult {
//         let (user_data, cb) = writer.into_callback();
//         unsafe { mlirExportVerilog(self, Some(cb), user_data) }
//     }
// }

// impl MlirLogicalResult {
//     pub fn is_ok(&self) -> bool {
//         self.value != 0
//     }
// }
// pub struct Context {
//     pub ctx: MlirContext,
// }

// impl Default for Context {
//     fn default() -> Self {
//         Self {
//             ctx: unsafe { mlirContextCreate() },
//         }
//     }
// }

// impl Context {
//     pub fn num_loaded_dialects(&self) -> isize {
//         unsafe { mlirContextGetNumLoadedDialects(self.ctx) as _ }
//     }
//     pub fn num_registered_dialects(&self) -> isize {
//         unsafe { mlirContextGetNumRegisteredDialects(self.ctx) as _ }
//     }
//     pub fn get_or_load_dialect(&self, namespace: &str) -> MlirDialect {
//         unsafe { mlirContextGetOrLoadDialect(self.ctx, namespace.into()) }
//     }
//     pub fn load_dialect(&self, namespace: &str) -> MlirDialect {
//         self.get_or_load_dialect(namespace.into())
//     }
//     pub fn new_unknown_location(&self) -> MlirLocation {
//         unsafe { mlirLocationUnknownGet(self.ctx) }
//     }
// }

// impl Drop for Context {
//     fn drop(&mut self) {
//         println!("Destroying Context");
//         unsafe { mlirContextDestroy(self.ctx) }
//     }
// }

// impl MlirDialectHandle {
//     pub fn load_dialect(self, context: &Context) -> Option<MlirDialect> {
//         let dialect = unsafe { mlirDialectHandleLoadDialect(self, context.ctx) };
//         if dialect.is_null() {
//             None
//         } else {
//             Some(dialect)
//         }
//     }
//     pub fn register_dialect(self, context: &Context) {
//         unsafe { mlirDialectHandleRegisterDialect(self, context.ctx) }
//     }
//     pub fn register_and_load(self, context: &Context) -> Option<MlirDialect> {
//         self.register_dialect(context);
//         self.load_dialect(context)
//     }
// }

// impl MlirOperation {
//     pub fn dump(self) {
//         unsafe { mlirOperationDump(self) }
//     }
// }

// impl MlirOperationState {
//     #[doc = "Constructs an operation state from a name and a location"]
//     pub fn get(name: &str, loc: MlirLocation) -> Self {
//         unsafe { mlirOperationStateGet(name.into(), loc) }
//     }
//     pub fn new_operation(&mut self) -> Option<MlirOperation> {
//         let op = unsafe { mlirOperationCreate(&mut *self) };
//         if op.is_null() {
//             None
//         } else {
//             Some(op)
//         }
//     }
// }
