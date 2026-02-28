use super::traits::Context;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ToolPreContext {
    pub tool_name: String,
    pub args: HashMap<String, String>,
}

impl Context for ToolPreContext {}

#[derive(Clone)]
pub struct ToolPostContext {
    pub tool_name: String,
    pub args: HashMap<String, String>,
    pub result: String,
    pub success: bool,
}

impl Context for ToolPostContext {}

#[derive(Clone)]
pub struct SessionStartContext {
    pub project_path: std::path::PathBuf,
    pub role: crate::types::Role,
}

impl Context for SessionStartContext {}

#[derive(Clone)]
pub struct SessionEndContext {
    pub turn_count: u32,
    pub project_path: std::path::PathBuf,
}

impl Context for SessionEndContext {}
