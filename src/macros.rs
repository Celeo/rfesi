/// Create a function for calling a single endpoint
/// with a GET request.
///
/// # Example
/// ```rust,no_run
/// # use rfesi::prelude::*
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     api_get!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         RequestType::Public,
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
/// # use rfesi::prelude::*;
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     api_get!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         RequestType::Public,
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
macro_rules! api_get {
    (
        $(#[$m:meta])*
        $fn_name:ident,
        $op_id:literal,
        $visibility:expr,
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
                query("GET", $visibility, &path, None, None)
                .await
        }
    };
}

/// Create a function for calling a single endpoint
/// with a POST request.
///
/// Follows the structure of the `api_get!` macro, with the
/// addition of taking an additional pair of `ident` and `ty`
/// to name and type the data that will be passed to
/// `serde_json::to_string` for serializing for setting the
/// request's body.
///
/// # Example
///
/// ```rust,no_run
/// # use rfesi::prelude::*
/// pub struct SomeGroup<'a> {
///     pub(crate) esi: &'a Esi,
/// }
///
/// impl<'a> SomeGroup<'a> {
///
///     api_post!(
///         /// Docs for the generated function
///         function_name,
///         "some_operation_id",
///         RequestType::Public,
///         Vec<u64>,
///         (alliance_id: u64) => "{alliance_id}",
///         ids: &[u64],
///     );
///
/// }
/// # fn main() {}
/// ```
/// ## Result:
///
/// ```rust,ignore
/// /// Docs for the generated function
/// pub async fn function_name(&self, alliance_id: u64, ids: &[u64]) -> EsiResult<Vec<u64>> {
///     let path = self.esi.get_endpoint_for_op_id("some_operation_id")?
///         .replace("{alliance_id}", &alliance_id.to_string());
///     let body = serde_json::to_string(ids);
///     self.esi
///         .query("GET", RequestType::Public, &path, None, Some(&body))
///         .await
/// }
/// ```
macro_rules! api_post {
    (
        $(#[$m:meta])*
        $fn_name:ident,
        $op_id:literal,
        $visibility:expr,
        $ret_type:ty,
        $( ($param:ident: $param_t:ty) => $replace:literal ),*,
        $body_param:ident: $param_type:ty,
    ) => {
        $(#[$m])*
        pub async fn $fn_name(&self, $( $param: $param_t, )* $body_param: $param_type) -> EsiResult<$ret_type> {
            let path = self
                .esi
                .get_endpoint_for_op_id($op_id)?
                $(
                    .replace($replace, &$param.to_string())
                )*;
            let body = serde_json::to_string($body_param)?;
            self.esi.
                query("GET", $visibility, &path, None, Some(&body))
                .await
        }
    }
}
