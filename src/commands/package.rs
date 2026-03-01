use crate::app::{AppResult, DbMessage, ErrorMessage};
use crate::{InstallOptions, RemoveOptions, SearchOptions, UpdateOptions, UpgradeOptions};

use upac_core_lib::{Backend, UpacConfig};

use std::sync::mpsc::Sender;

pub(crate) fn install(
    opts: InstallOptions,
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
    opts: RemoveOptions,
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

pub(crate) fn update(
    opts: UpdateOptions,
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

pub(crate) fn upgrade(
    opts: UpgradeOptions,
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

pub(crate) fn search(
    opts: SearchOptions,
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

pub(crate) fn show(
    package: &str,
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

pub(crate) fn files(
    package: &str,
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

pub(crate) fn deps(
    package: &str,
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
