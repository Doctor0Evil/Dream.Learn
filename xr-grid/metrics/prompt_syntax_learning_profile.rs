use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PromptSyntaxLearningProfile {
    pub user_id: String, // Bostrom / Sync-ID alias, infra-only
    pub syntax_families: HashMap<String, f32>,  // e.g. "math" -> 0.7
    pub metric_density: f32,                    // metrics per 1k tokens
    pub safety_clause_rate: f32,                // 0.0–1.0
    pub shard_links: Vec<String>,               // hex IDs of LearningShards
}

// Hex-stamp: PromptSyntaxLearningProfile v1
// 0x94c7a1e3d2b5490fa6c3e9f105bd7c92
