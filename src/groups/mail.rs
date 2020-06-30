#![allow(unused)]

use crate::Esi;

pub struct MailGroup<'a> {
    pub(crate) esi: &'a Esi,
}
