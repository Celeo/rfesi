/// Create a function for calling a single endpoint
/// with a GET request with no extra processing on
/// the operation ID or path.
///
/// # Example
/// ```rust,no_run
/// # use rfesi::{simple_get, Esi, EsiResult, RequestType};
/// # use serde::Deserialize;
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     simple_get!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         Vec<u64>
///     );
///
/// }
/// # fn main() {}
/// ```
///
/// ## Result:
///
/// ```rust,ignore
/// /// Docs for the generated function
/// pub async fn function_name(&self) -> EsiResult<Vec<u64>> {
///     let path = self.esi.get_endpoint_for_op_id("some_operation_id")?;
///     self.esi
///         .query("GET", RequestType::Public, &path, None, None)
///         .await
/// }
/// ```
#[macro_export]
macro_rules! simple_get {
    (
        $(#[$m:meta])*
        $fn_name:ident,
        $op_id:literal,
        $ret_type:ty
    ) => {
        $(#[$m])*
        pub async fn $fn_name(&self) -> EsiResult<$ret_type> {
            let path = self.esi.get_endpoint_for_op_id($op_id)?;
            self.esi
                .query("GET", RequestType::Public, &path, None, None)
                .await
        }
    }
}
