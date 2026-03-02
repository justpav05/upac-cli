use crate::app::{AppResult, AppError};
use crate::{InstallOptions, RemoveOptions, SearchOptions, UpdateOptions, UpgradeOptions};

use upac_core_lib::{Backend, Database, Install, Installer, OStreeRepo, PackageDiff, PackageRegistry, PackageRepo, UpacConfig};

use std::path::Path;

pub(crate) fn install(
    options: InstallOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
    	// Проверка существования пути пакета
        if !&options.package.exists() {
            return Err(AppError::CommandError(
                format!("File not found: {}", &options.package.display())
            ));
        }

        // Выбор бекенда для пакета
        let backend = backends.iter().find(|backend| backend.detect(&options.package)).ok_or_else(|| AppError::CommandError(format!("Unsupported package format: {}", &options.package.display())))?;

        // Извлекаем пакет во временную директорию
        let extracted_package = backend.extract(&options.package, &config.temp_dir).map_err(|err| AppError::CommandError(err.to_string()))?;

        // Устанавливаем
        installer.install(&extracted_package).map_err(|err| AppError::CommandError(err.to_string()))?;

        // Если ostree включён — делаем коммит
        if config.ostree.enabled {
            let packages = installer.list_packages().map_err(|err| AppError::CommandError(err.to_string()))?;

            let mut packages_files = Vec::new();
            for package in &packages {
                packages_files.extend(installer.list_files(&package.name).map_err(|err| AppError::CommandError(err.to_string()))?);
            }

            let diff = PackageDiff {
                added:   vec![extracted_package.name.clone()],
                removed: vec![],
                updated: vec![],
            };

            ostree.ok_or_else(|| AppError::CommandError(String::from("OSTree not available")))?.commit(packages_files, &diff).map_err(|err| AppError::CommandError(err.to_string()))?;
        }

        Ok(())
    }
}

pub(crate) fn remove(
    options: RemoveOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        let package = database.get_package(&options.package).map_err(|err| AppError::CommandError(err.to_string()))?.ok_or_else(|| AppError::CommandError(format!("Package not found: {}", options.package)))?;

        installer.remove(&package.name).map_err(|err| AppError::CommandError(err.to_string()))?;

        if config.ostree.enabled {
            let packages = installer.list_packages().map_err(|err| AppError::CommandError(err.to_string()))?;

            let mut packages_files = Vec::new();
            for package in &packages {
                packages_files.extend(installer.list_files(&package.name).map_err(|err| AppError::CommandError(err.to_string()))?);
            }

            let diff = PackageDiff {
                added:   vec![],
                removed: vec![options.package.clone()],
                updated: vec![],
            };

            ostree.ok_or_else(|| AppError::CommandError(String::from("OSTree not available")))?.commit(packages_files, &diff).map_err(|err| AppError::CommandError(err.to_string()))?;
        }

        Ok(())
    }
}

pub(crate) fn update(
    options: UpdateOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}

pub(crate) fn upgrade(
    options: UpgradeOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}

pub(crate) fn search(
    options: SearchOptions,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}

pub(crate) fn show(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}

pub(crate) fn files(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}

pub(crate) fn deps(
	package: &str,
) -> impl FnOnce(
    &mut Installer,
    Option<&OStreeRepo>,
    &UpacConfig,
    &Database,
    &[Box<dyn Backend>],
) -> AppResult<()> {
    move |installer, ostree, config, database, backends| {
        todo!()
    }
}
