use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct LoadBalancer {
    nodes: BTreeMap<u64, String>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer { nodes: BTreeMap::new() }
    }

    pub fn add_node(&mut self, node_id: &str) {
        let hash = self.hash_key(node_id);
        self.nodes.insert(hash, node_id.to_string());
    }

    pub fn get_node(&self, request_id: &str) -> Option<&String> {
        if self.nodes.is_empty() { return None; }
        let hash = self.hash_key(request_id);
        match self.nodes.range(hash..).next() {
            Some((_, node)) => Some(node),
            None => Some(self.nodes.values().next().unwrap()),
        }
    }

    fn hash_key(&self, key: &str) -> u64 {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish()
    }
}
