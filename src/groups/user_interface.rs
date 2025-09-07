use crate::prelude::*;

/// Endpoints for UserInterface
pub struct UserInterfaceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl UserInterfaceGroup<'_> {
    /// Open the market details window.
    pub async fn open_market_details_window(
        &self,
        character_id: u64,
        type_id: i32,
    ) -> EsiResult<()> {
        // not using the macro since it doesn't like no body
        let path = self
            .esi
            .get_endpoint_for_op_id("post_ui_openwindow_marketdetails")?
            .replace("{character_id}", &character_id.to_string())
            .replace("{type_id}", &type_id.to_string());
        self.esi
            .query("POST", RequestType::Authenticated, &path, None, None)
            .await
    }
}
