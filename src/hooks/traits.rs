use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Hook<C: Context>: Send + Sync {
    async fn call(&self, ctx: C, next: Next<'_, C>) -> Result<HookResult>;
}

pub trait Context: Send + Sync {}

pub enum HookResult {
    Continue,
    Block { reason: String },
}

pub struct Next<'a, C: Context> {
    chain: &'a HookChain<C>,
    index: usize,
}

impl<'a, C: Context> Next<'a, C> {
    pub async fn run(self, ctx: C) -> Result<HookResult> {
        self.chain.run_hook(ctx, self.index).await
    }
}

pub struct HookChain<C: Context> {
    hooks: Vec<Box<dyn Hook<C>>>,
}

impl<C: Context> HookChain<C> {
    pub fn new(hooks: Vec<Box<dyn Hook<C>>>) -> Self {
        Self { hooks }
    }

    pub async fn execute(&self, ctx: C) -> Result<HookResult> {
        self.run_hook(ctx, 0).await
    }

    pub(crate) async fn run_hook(&self, ctx: C, index: usize) -> Result<HookResult> {
        if let Some(hook) = self.hooks.get(index) {
            let next = Next {
                chain: self,
                index: index + 1,
            };
            hook.call(ctx, next).await
        } else {
            Ok(HookResult::Continue)
        }
    }
}
