use anyhow::{bail, Result};
use indoc::formatdoc;
use turbo_tasks::{RcStr, Vc};
use turbo_tasks_fs::FileSystemPath;
use turbopack_core::{
    asset::{Asset, AssetContent},
    chunk::{ChunkItem, ChunkItemExt, ChunkType, ChunkableModule, ChunkingContext},
    ident::AssetIdent,
    module::Module,
    reference::ModuleReferences,
};
use turbopack_ecmascript::{
    chunk::{
        EcmascriptChunkItem, EcmascriptChunkItemContent, EcmascriptChunkPlaceable,
        EcmascriptChunkType, EcmascriptExports,
    },
    references::esm::EsmExports,
    utils::StringifyJs,
};

use super::server_component_reference::NextServerComponentModuleReference;

#[turbo_tasks::function]
fn modifier() -> Vc<RcStr> {
    Vc::cell("Next.js server component".into())
}

#[turbo_tasks::value(shared)]
pub struct NextServerComponentModule {
    module: Vc<Box<dyn EcmascriptChunkPlaceable>>,
}

#[turbo_tasks::value_impl]
impl NextServerComponentModule {
    #[turbo_tasks::function]
    pub fn new(module: Vc<Box<dyn EcmascriptChunkPlaceable>>) -> Vc<Self> {
        NextServerComponentModule { module }.cell()
    }

    #[turbo_tasks::function]
    pub async fn server_path(self: Vc<Self>) -> Result<Vc<FileSystemPath>> {
        let this = self.await?;
        Ok(this.module.ident().path())
    }
}

#[turbo_tasks::value_impl]
impl Module for NextServerComponentModule {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.module.ident().with_modifier(modifier())
    }

    #[turbo_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        Vc::cell(vec![Vc::upcast(NextServerComponentModuleReference::new(
            Vc::upcast(self.module),
        ))])
    }
}

#[turbo_tasks::value_impl]
impl Asset for NextServerComponentModule {
    #[turbo_tasks::function]
    fn content(&self) -> Result<Vc<AssetContent>> {
        bail!("Next.js server component module has no content")
    }
}

#[turbo_tasks::value_impl]
impl ChunkableModule for NextServerComponentModule {
    #[turbo_tasks::function]
    async fn as_chunk_item(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Result<Vc<Box<dyn turbopack_core::chunk::ChunkItem>>> {
        Ok(Vc::upcast(
            BuildServerComponentChunkItem {
                chunking_context,
                inner: self,
            }
            .cell(),
        ))
    }
}

#[turbo_tasks::value_impl]
impl EcmascriptChunkPlaceable for NextServerComponentModule {
    #[turbo_tasks::function]
    fn get_exports(&self) -> Vc<EcmascriptExports> {
        let module_reference = Vc::upcast(NextServerComponentModuleReference::new(Vc::upcast(
            self.module,
        )));

        EcmascriptExports::EsmExports(
            EsmExports {
                exports: Default::default(),
                star_exports: vec![module_reference],
            }
            .cell(),
        )
        .cell()
    }
}

#[turbo_tasks::value]
struct BuildServerComponentChunkItem {
    chunking_context: Vc<Box<dyn ChunkingContext>>,
    inner: Vc<NextServerComponentModule>,
}

#[turbo_tasks::value_impl]
impl EcmascriptChunkItem for BuildServerComponentChunkItem {
    #[turbo_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        self.chunking_context
    }

    #[turbo_tasks::function]
    async fn content(self: Vc<Self>) -> Result<Vc<EcmascriptChunkItemContent>> {
        let this = self.await?;
        let inner = this.inner.await?;

        let module_id = inner
            .module
            .as_chunk_item(Vc::upcast(this.chunking_context))
            .id()
            .await?;
        Ok(EcmascriptChunkItemContent {
            inner_code: formatdoc!(
                r#"
                    __turbopack_export_namespace__(__turbopack_import__({}));
                "#,
                StringifyJs(&module_id),
            )
            .into(),
            ..Default::default()
        }
        .cell())
    }
}

#[turbo_tasks::value_impl]
impl ChunkItem for BuildServerComponentChunkItem {
    #[turbo_tasks::function]
    fn asset_ident(&self) -> Vc<AssetIdent> {
        self.inner.ident()
    }

    #[turbo_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        self.inner.references()
    }

    #[turbo_tasks::function]
    async fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        self.chunking_context
    }

    #[turbo_tasks::function]
    fn ty(&self) -> Vc<Box<dyn ChunkType>> {
        Vc::upcast(Vc::<EcmascriptChunkType>::default())
    }

    #[turbo_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        Vc::upcast(self.inner)
    }
}
