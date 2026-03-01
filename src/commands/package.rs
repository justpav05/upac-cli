use crate::app::{AppResult, AppError};
use crate::{InstallOptions, RemoveOptions, SearchOptions, UpdateOptions, UpgradeOptions};

use upac_core_lib::{Backend, UpacConfig, Installer, Install, OStreeRepo, PackageRepo, PackageDiff};

use std::path::Path;

pub(crate) fn install(
    opts: InstallOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        let path = Path::new(&opts.package);

        if !path.exists() {
            return Err(AppError::CommandError(
                format!("File not found: {}", path.display())
            ));
        }

        // Ищем подходящий бэкенд
        let backend = backends
            .iter()
            .find(|backend| backend.detect(path))
            .ok_or_else(|| AppError::CommandError(
                format!("Unsupported package format: {}", path.display())
            ))?;

        // Извлекаем пакет во временную директорию
        let extracted = backend
            .extract(path, &config.temp_dir)
            .map_err(|err| AppError::CommandError(err.to_string()))?;

        // Устанавливаем
        installer
            .install(&extracted)
            .map_err(|err| AppError::CommandError(err.to_string()))?;

        // Если ostree включён — делаем коммит
        if let Some(ostree) = ostree {
            let packages = installer
                .list_packages()
                .map_err(|err| AppError::CommandError(err.to_string()))?;

            let mut files = Vec::new();
            for pkg in &packages {
                files.extend(installer.list_files(&pkg.name).map_err(|err| AppError::CommandError(err.to_string()))?);
            }

            let diff = PackageDiff {
                added:   vec![extracted.name.clone()],
                removed: vec![],
                updated: vec![],
            };

            ostree
                .commit(files, &diff)
                .map_err(|err| AppError::CommandError(err.to_string()))?;
        }

        Ok(())
    }
}

pub(crate) fn remove(
    opts: RemoveOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        let packages = installer.list_packages().map_err(|err| AppError::CommandError(err.to_string()))?;

        if !packages.iter().any(|p| p.name == opts.package) {
            return Err(AppError::CommandError(
                format!("Package not found: {}", opts.package)
            ));
        }

        // Удаляем пакет
        installer.remove(&opts.package).map_err(|e| AppError::CommandError(e.to_string()))?;

        if let Some(ostree) = ostree {
            let packages = installer
                .list_packages()
                .map_err(|err| AppError::CommandError(err.to_string()))?;

            let mut files = Vec::new();
            for pkg in &packages {
                files.extend(installer.list_files(&pkg.name).map_err(|err| AppError::CommandError(err.to_string()))?);
            }

            let diff = PackageDiff {
                added:   vec![],
                removed: vec![opts.package.clone()],
                updated: vec![],
            };

            ostree
                .commit(files, &diff)
                .map_err(|e| AppError::CommandError(e.to_string()))?;
        }

        Ok(())
    }
}

pub(crate) fn update(
    opts: UpdateOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}

pub(crate) fn upgrade(
    opts: UpgradeOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}

pub(crate) fn search(
    opts: SearchOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}

pub(crate) fn show(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}

pub(crate) fn files(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}

pub(crate) fn deps(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, backends| {
        todo!()
    }
}
