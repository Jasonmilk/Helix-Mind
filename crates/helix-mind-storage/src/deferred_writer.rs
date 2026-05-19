pub struct DeferredWriter;

impl DeferredWriter {
    pub fn new() -> Self {
        Self
    }

    pub async fn push(&self, _node: helix_mind_core::graph::Node) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn run(&self) {
        todo!()
    }
}