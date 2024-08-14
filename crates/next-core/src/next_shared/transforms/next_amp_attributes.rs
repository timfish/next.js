use anyhow::Result;
use async_trait::async_trait;
use next_custom_transforms::transforms::amp_attributes::amp_attributes;
use swc_core::{
    common::util::take::Take,
    ecma::{ast::*, visit::FoldWith},
};
use turbo_tasks::Vc;
use turbopack::module_options::{ModuleRule, ModuleRuleEffect};
use turbopack_ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext};

use super::module_rule_match_js_no_url;

pub fn get_next_amp_attr_rule(enable_mdx_rs: bool) -> ModuleRule {
    let transformer =
        EcmascriptInputTransform::Plugin(Vc::cell(Box::new(NextAmpAttributes {}) as _));
    ModuleRule::new(
        module_rule_match_js_no_url(enable_mdx_rs),
        vec![ModuleRuleEffect::ExtendEcmascriptTransforms {
            prepend: Vc::cell(vec![]),
            append: Vc::cell(vec![transformer]),
        }],
    )
}

#[derive(Debug)]
struct NextAmpAttributes {}

#[async_trait]
impl CustomTransformer for NextAmpAttributes {
    #[tracing::instrument(level = tracing::Level::TRACE, name = "next_amp", skip_all)]
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));
        *program = p.fold_with(&mut amp_attributes());
        Ok(())
    }
}
