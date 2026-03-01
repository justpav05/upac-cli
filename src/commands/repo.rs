use crate::app::{AppResult, ErrorMessage};

use upac_core_lib::{Backend, Installer, OStreeRepo, UpacConfig};

use std::sync::mpsc::Sender;

pub(crate) fn add(
    url: String,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| todo!()
}

pub(crate) fn remove(
    url: String,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| todo!()
}

pub(crate) fn update() -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| todo!()
}
