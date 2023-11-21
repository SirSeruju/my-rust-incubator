pub mod my_error;
pub mod my_iterator_ext;

// compiler says:
// note: `MyIteratorExt` is a "sealed trait",
// because to implement it you also need to implement `my_iterator_ext::private::Sealed`,
// which is not accessible; this is usually done to force you to use one of the provided types that already implement it
//
// struct MyIterator;
// impl MyIteratorExt for MyIterator {}

/// compiler says:
/// note: `MyIteratorExt` is a "sealed trait",
/// because to implement it you also need to implement `my_iterator_ext::private::Sealed`,
/// which is not accessible; this is usually done to force you to use one of the provided types that already implement it
///
/// ```compile_fail
/// use step_2_6::MyIteratorExt;
///
/// struct MyIterator;
/// impl MyIteratorExt for MyIterator {}
/// ```
pub use self::my_iterator_ext::MyIteratorExt;

// module `private` is private
// so we cant use private::Token to redefine `type_id` method
// use self::my_error::private;
//
// impl MyError for u8 {
//     fn type_id(&self, _: private::Token) -> std::any::TypeId
//         where
//             Self: 'static {
//                 todo!()
//     }
// }
// but still can redefine `source` method
// impl MyError for u8 {
//     fn source(&self) -> Option<&(dyn MyError + 'static)> {
//         None
//     }
// }

/// module `private` is private
/// so we cant use private::Token to redefine `type_id` method
/// ```compile_fail
/// use step_2_6::MyError;
///
/// impl MyError for u8 {
///     fn type_id(&self, _: private::Token) -> std::any::TypeId
///         where
///             Self: 'static {
///                 todo!()
///     }
/// }
/// ```
/// but still can redefine `source` method
/// ```
/// use std::fmt::Display;
/// use step_2_6::MyError;
///
/// #[derive(Debug)]
/// struct t;
///
/// impl Display for t {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
/// impl MyError for t {
///     fn source(&self) -> Option<&(dyn MyError + 'static)> {
///         None
///     }
/// }
/// ```
pub use self::my_error::MyError;
