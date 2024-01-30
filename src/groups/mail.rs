#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Mail
pub struct MailGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Information about all mail labels.
pub struct MailLabels {
    /// List of individual mail labels.
    pub labels: Vec<MailLabel>,
    /// Total unread count across all labels.
    pub unread_count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Information about an individual mail label.
pub struct MailLabel {
    /// Color of the label as RGB Hex (`#rrggbb`).
    pub color: String,
    /// ID of the label.
    pub label_id: i32,
    /// Name of the label.
    pub name: String,
    /// Number of unread messages with this label.
    #[serde(default)]
    pub unread_count: Option<i32>,
}

impl<'a> MailGroup<'a> {
    api_get!(
        /// Return a list of the users mail labels, unread counts for each
        /// label and a total unread count.
        get_character_mail_labels,
        "get_characters_character_id_mail_labels",
        RequestType::Authenticated,
        MailLabels,
        (character_id: i32) => "{character_id}"
    );
}
