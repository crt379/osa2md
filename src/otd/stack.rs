use std::sync::RwLock;

use super::context::Context;

#[derive(Debug)]
pub struct Stack {
    contexts: Vec<RwLock<Context>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            contexts: Default::default(),
        }
    }

    pub fn push(&mut self, context: Context) {
        self.contexts.push(RwLock::new(context));
    }

    pub fn pop(&mut self) -> Option<RwLock<Context>> {
        self.contexts.pop()
    }

    pub fn first(&self) -> Option<&RwLock<Context>> {
        self.contexts.first()
    }

    pub fn last(&self) -> Option<&RwLock<Context>> {
        self.contexts.last()
    }
}
