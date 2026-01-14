// Stub module for removed DAP debug functionality.
// Contains minimal types preserved to maintain terminal and run functionality.
// DAP debugging has been fully removed.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    path::PathBuf,
    rc::Rc,
    time::Instant,
};

use floem::reactive::{RwSignal, Scope};
use lapce_rpc::{
    dap_types::{self, DapId, StackFrame, Stopped, ThreadId, Variable},
    terminal::TermId,
};
use serde::{Deserialize, Serialize};

use crate::window_tab::CommonData;

/// Run mode for terminal processes (Debug removed - only Run remains)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunDebugMode {
    Run,
}

impl Display for RunDebugMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunDebugMode::Run => write!(f, "run"),
        }
    }
}

/// Process run state tracking
#[derive(Clone)]
pub struct RunDebugProcess {
    pub mode: RunDebugMode,
    pub config: lapce_rpc::dap_types::RunDebugConfig,
    pub is_prelaunch: bool,
    pub stopped: bool,
    pub created: Instant,
}

/// Stub for breakpoint (DAP removed)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LapceBreakpoint {
    pub id: Option<usize>,
    pub line: usize,
    pub offset: usize,
    pub active: bool,
    pub verified: Option<bool>,
    pub message: Option<String>,
    pub dap_verified: Option<bool>,
    pub dap_line: Option<usize>,
}

/// Stub for run debug configs list
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RunDebugConfigs {
    pub configs: Vec<lapce_rpc::dap_types::RunDebugConfig>,
}

/// Stub for run debug data container
#[derive(Clone)]
pub struct RunDebugData {
    pub breakpoints: RwSignal<BTreeMap<PathBuf, BTreeMap<usize, LapceBreakpoint>>>,
    pub active_term: RwSignal<Option<TermId>>,
    pub daps: RwSignal<im::HashMap<DapId, DapData>>,
}

impl RunDebugData {
    pub fn new(
        cx: Scope,
        breakpoints: RwSignal<BTreeMap<PathBuf, BTreeMap<usize, LapceBreakpoint>>>,
    ) -> Self {
        Self {
            breakpoints,
            active_term: cx.create_rw_signal(None),
            daps: cx.create_rw_signal(im::HashMap::new()),
        }
    }

    pub fn source_breakpoints(
        &self,
    ) -> HashMap<PathBuf, Vec<dap_types::SourceBreakpoint>> {
        HashMap::new()
    }
}

/// Stub for DAP data (minimal - DAP removed)
#[derive(Clone)]
pub struct DapData {
    pub dap_id: DapId,
    pub term_id: TermId,
    pub stopped: RwSignal<bool>,
    pub thread_id: RwSignal<Option<ThreadId>>,
    pub breakline: RwSignal<Option<(usize, PathBuf)>>,
    pub variables: RwSignal<DapVariable>,
    pub common: Rc<CommonData>,
}

impl DapData {
    pub fn new(
        cx: Scope,
        dap_id: DapId,
        term_id: TermId,
        common: Rc<CommonData>,
    ) -> Self {
        Self {
            dap_id,
            term_id,
            stopped: cx.create_rw_signal(false),
            thread_id: cx.create_rw_signal(None),
            breakline: cx.create_rw_signal(None),
            variables: cx.create_rw_signal(DapVariable::default()),
            common,
        }
    }

    /// Stub - DAP stopped handling removed
    pub fn stopped(
        &self,
        _cx: Scope,
        _stopped: &Stopped,
        _stack_frames: &HashMap<ThreadId, Vec<StackFrame>>,
        _variables: &[(dap_types::Scope, Vec<Variable>)],
    ) {
        // DAP debugging removed - no-op
    }
}

/// Stub for scope or var enum
#[derive(Clone, PartialEq, Eq)]
pub enum ScopeOrVar {
    Scope(lapce_rpc::dap_types::Scope),
    Var(lapce_rpc::dap_types::Variable),
}

impl Default for ScopeOrVar {
    fn default() -> Self {
        Self::Scope(lapce_rpc::dap_types::Scope::default())
    }
}

impl ScopeOrVar {
    pub fn name(&self) -> &str {
        ""
    }

    pub fn value(&self) -> Option<&str> {
        None
    }

    pub fn ty(&self) -> Option<&str> {
        None
    }

    pub fn reference(&self) -> usize {
        0
    }
}

/// Stub for DAP variable view
#[derive(Clone, Default)]
pub struct DapVariable {
    pub item: ScopeOrVar,
    pub parent: Vec<usize>,
    pub expanded: bool,
    pub read: bool,
    pub children: Vec<DapVariable>,
    pub children_expanded_count: usize,
}

impl DapVariable {
    pub fn total_len(&self) -> usize {
        0
    }
}
