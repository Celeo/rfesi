use crate::prelude::*;
use std::collections::HashMap;

/// Endpoints for UserInterface
pub struct UserInterfaceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl UserInterfaceGroup<'_> {
    /// Open the market details window.
    pub async fn open_market_details_window(
        &self,
        type_id: i32,
        compatibility_date: &str,
    ) -> EsiResult<()> {
        // not using the macro since it doesn't like no body
        let path = self
            .esi
            .get_endpoint_for_op_id("post_ui_openwindow_marketdetails")?
            .replace("{type_id}", &type_id.to_string());
        let mut headers = HashMap::new();
        headers.insert("X-Compatibility-Date", compatibility_date.to_string());
        self.esi
            .query(
                "POST",
                RequestType::Authenticated,
                &path,
                None,
                None,
                Some(headers),
            )
            .await
    }
}
