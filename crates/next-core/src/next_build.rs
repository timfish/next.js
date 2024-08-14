use anyhow::Result;
use turbo_tasks::{RcStr, Vc};
use turbo_tasks_fs::FileSystemPath;
use turbopack_core::resolve::{options::ImportMapping, ExternalType};

use crate::next_import_map::get_next_package;

#[turbo_tasks::function]
pub async fn get_postcss_package_mapping(
    project_path: Vc<FileSystemPath>,
) -> Result<Vc<ImportMapping>> {
    Ok(ImportMapping::Alternatives(vec![
        // Prefer the local installed version over the next.js version
        ImportMapping::PrimaryAlternative("postcss".into(), Some(project_path)).cell(),
        ImportMapping::PrimaryAlternative("postcss".into(), Some(get_next_package(project_path)))
            .cell(),
    ])
    .cell())
}

#[turbo_tasks::function]
pub async fn get_external_next_compiled_package_mapping(
    package_name: Vc<RcStr>,
) -> Result<Vc<ImportMapping>> {
    Ok(ImportMapping::Alternatives(vec![ImportMapping::External(
        Some(format!("next/dist/compiled/{}", &*package_name.await?).into()),
        ExternalType::CommonJs,
    )
    .into()])
    .cell())
}
