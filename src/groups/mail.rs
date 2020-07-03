#![allow(unused)]

use crate::Esi;

/// Endpoints for Mail
pub struct MailGroup<'a> {
    pub(crate) esi: &'a Esi,
}
