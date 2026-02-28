use chrono_h::hooks::{Hook, HookChain, HookResult, Next, ToolPreContext};
use async_trait::async_trait;

struct TestHook {
    name: String,
    should_block: bool,
}

#[async_trait]
impl Hook<ToolPreContext> for TestHook {
    async fn call(
        &self,
        ctx: ToolPreContext,
        next: Next<'_, ToolPreContext>,
    ) -> chrono_h::Result<HookResult> {
        if self.should_block {
            return Ok(HookResult::Block {
                reason: format!("Blocked by {}", self.name),
            });
        }
        next.run(ctx).await
    }
}

#[tokio::test]
async fn test_hook_chain_continues() {
    let hooks: Vec<Box<dyn Hook<ToolPreContext>>> = vec![
        Box::new(TestHook {
            name: "first".to_string(),
            should_block: false,
        }),
        Box::new(TestHook {
            name: "second".to_string(),
            should_block: false,
        }),
    ];
    
    let chain = HookChain::new(hooks);
    let ctx = ToolPreContext {
        tool_name: "read".to_string(),
        args: Default::default(),
    };
    
    let result = chain.execute(ctx).await.unwrap();
    assert!(matches!(result, HookResult::Continue));
}

#[tokio::test]
async fn test_hook_chain_blocks() {
    let hooks: Vec<Box<dyn Hook<ToolPreContext>>> = vec![
        Box::new(TestHook {
            name: "first".to_string(),
            should_block: true,
        }),
        Box::new(TestHook {
            name: "second".to_string(),
            should_block: false,
        }),
    ];
    
    let chain = HookChain::new(hooks);
    let ctx = ToolPreContext {
        tool_name: "read".to_string(),
        args: Default::default(),
    };
    
    let result = chain.execute(ctx).await.unwrap();
    assert!(matches!(result, HookResult::Block { .. }));
}
