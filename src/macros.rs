/// Create a function for calling a single endpoint
/// with a GET request.
///
/// TODO
/// Note that this isn't useful for any other crate,
/// but I haven't been able to figure out how to limit
/// the macro to just this crate, so here it lies.
///
/// # Example
/// ```rust,no_run
/// # use rfesi::{http_get, Esi, EsiResult, RequestType};
/// # use serde::Deserialize;
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     http_get!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         Vec<u64>,
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
///
/// Additionally, this macro supports path replacements to insert variables
/// into the path from ESI.
///
/// # Example
///
/// ```rust,no_run
/// # use rfesi::{http_get, Esi, EsiResult, RequestType};
/// # use serde::Deserialize;
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     http_get!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         Vec<u64>,
///         (alliance_id: u64) => "{alliance_id}"
///     );
///
/// }
/// # fn main() {}
/// ```
/// ## Result:
///
/// ```rust,ignore
/// /// Docs for the generated function
/// pub async fn function_name(&self, alliance_id: u64) -> EsiResult<Vec<u64>> {
///     let path = self.esi.get_endpoint_for_op_id("some_operation_id")?
///         .replace("{alliance_id}", &alliance_id.to_string());
///     self.esi
///         .query("GET", RequestType::Public, &path, None, None)
///         .await
/// }
/// ```
#[macro_export]
macro_rules! http_get {
    (
        $(#[$m:meta])*
        $fn_name:ident,
        $op_id:literal,
        $ret_type:ty,
        $( ($param:ident: $param_t:ty) => $replace:literal ),*
    ) => {
        $(#[$m])*
        pub async fn $fn_name(&self, $( $param: $param_t, )*) -> EsiResult<$ret_type> {
            let path = self
                .esi
                .get_endpoint_for_op_id($op_id)?
                $(
                    .replace($replace, &$param.to_string())
                )*;
            self.esi.
                query("GET", RequestType::Public, &path, None, None)
                .await
        }
    };
}
