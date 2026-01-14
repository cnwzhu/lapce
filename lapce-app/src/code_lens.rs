// Stub module for code lens functionality.
// Code lens provides Run/Debug buttons above code, but since DAP is removed,
// this module provides a no-op stub.

use serde_json::Value;
use std::rc::Rc;

use crate::window_tab::CommonData;

/// Code lens data stub - Run/Debug code lens removed
#[derive(Clone)]
pub struct CodeLensData {
    _common: Rc<CommonData>,
}

impl CodeLensData {
    pub fn new(common: Rc<CommonData>) -> Self {
        Self { _common: common }
    }

    /// No-op run - DAP has been removed
    pub fn run(&self, _command: &str, _args: Vec<Value>) {
        // Code lens run functionality disabled - DAP removed
    }
}
