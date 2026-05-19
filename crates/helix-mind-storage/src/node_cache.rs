use helix_mind_core::graph::Node;
use moka::sync::Cache;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Clone)]
pub struct NodeCache {
    cache: Arc<Cache<Uuid, Node>>,
}

impl NodeCache {
    pub fn new(capacity: u64) -> Self {
        let cache = Cache::new(capacity);
        Self { cache: Arc::new(cache) }
    }

    pub fn get(&self, id: &Uuid) -> Option<Node> {
        self.cache.get(id)
    }

    pub fn put(&self, node: Node) {
        self.cache.insert(node.id, node);
    }

    pub fn invalidate(&self, id: &Uuid) {
        self.cache.invalidate(id);
    }

    pub fn clear(&self) {
        self.cache.invalidate_all();
    }
}
