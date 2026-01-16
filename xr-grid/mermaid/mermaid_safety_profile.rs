pub struct MermaidSafetyProfile {
    pub max_nodes: u32,
    pub max_edges: u32,
    pub allow_state_machines: bool,
    pub require_neurorights_labels: bool, // e.g. mentalPrivacy, noPersonScoring
    pub infra_only_flag: bool,            // must be true: diagrams describe infra, never persons
}

// Coupling rule sketch (conceptual):
// if PromptSyntaxLearningProfile.metric_density < 0.5 or safety_clause_rate < 0.6:
//     MermaidSafetyProfile.max_nodes = 12; allow_state_machines = false;
// else:
//     MermaidSafetyProfile.max_nodes = 40; allow_state_machines = true;

// Hex-stamp: MermaidSafetyProfile v1
// 0xa1d7c3f9b2044e8ab5c9e1d37f6a9c52
