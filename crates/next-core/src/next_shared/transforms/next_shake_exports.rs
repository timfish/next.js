use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::shake_exports::{shake_exports, Config};
use swc_core::{
    common::util::take::Take,
    ecma::{ast::*, visit::FoldWith},
};
use turbo_tasks::Vc;
use turbopack::module_options::{ModuleRule, ModuleRuleEffect};
use turbopack_ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext};

use super::module_rule_match_js_no_url;

#[allow(dead_code)]
pub fn get_next_shake_exports_rule(enable_mdx_rs: bool, ignore: Vec<String>) -> ModuleRule {
    let transformer =
        EcmascriptInputTransform::Plugin(Vc::cell(Box::new(NextShakeExports { ignore }) as _));
    ModuleRule::new(
        module_rule_match_js_no_url(enable_mdx_rs),
        vec![ModuleRuleEffect::ExtendEcmascriptTransforms {
            prepend: Vc::cell(vec![]),
            append: Vc::cell(vec![transformer]),
        }],
    )
}

#[derive(Debug)]
struct NextShakeExports {
    ignore: Vec<String>,
}

#[async_trait]
impl CustomTransformer for NextShakeExports {
    #[tracing::instrument(level = tracing::Level::TRACE, name = "next_shake_exports", skip_all)]
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));

        *program = p.fold_with(&mut shake_exports(Config {
            ignore: self.ignore.iter().map(|s| s.clone().into()).collect(),
        }));
        Ok(())
    }
}
