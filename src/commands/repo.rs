use crate::app::{AppResult, DbMessage, ErrorMessage};

use upac_core_lib::{Backend, UpacConfig};

use std::sync::mpsc::Sender;

pub(crate) fn add(
    url: &str,
) -> impl FnOnce(
    &Sender<DbMessage>,
    &Sender<ErrorMessage>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()>
       + Send
       + 'static {
    move |database_tx, error_tx, config, _backends| todo!()
}

pub(crate) fn remove(
    url: &str,
) -> impl FnOnce(
    &Sender<DbMessage>,
    &Sender<ErrorMessage>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()>
       + Send
       + 'static {
    move |database_tx, error_tx, config, _backends| todo!()
}

pub(crate) fn update() -> impl FnOnce(
    &Sender<DbMessage>,
    &Sender<ErrorMessage>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()>
       + Send
       + 'static {
    move |database_tx, error_tx, config, _backends| todo!()
}
