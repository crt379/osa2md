use std::rc::Rc;

use serde_json::Value;

use super::{
    context::{Context, CtxValue},
    otd::Otd,
};

pub enum RunState {
    Break,
    Continue,
    Variables(Vec<(String, Rc<CtxValue>)>),
    Return,
    None,
}

pub trait IFunc {
    fn get(&self, name: &str)
        -> Option<fn(ctx: Context, otd: &Otd, ifunc: &dyn IFunc) -> RunState>;
}

pub struct Exec {
    ctx: Context,
    otds: Vec<Otd>,
    ifunc: Box<dyn IFunc>,
}

impl Exec {
    pub fn new(otds: Vec<Otd>, basics: Value, ifunc: Box<dyn IFunc>) -> Self {
        Self {
            ctx: Context::new(basics),
            otds,
            ifunc,
        }
    }

    pub fn run(&mut self) {
        for otd in self.otds.iter() {
            match self.ifunc.get(&otd.func).unwrap()(self.ctx.son(), otd, self.ifunc.as_ref()) {
                RunState::Variables(vs) => {
                    for (n, v) in vs {
                        self.ctx.insert(n, v.clone());
                    }
                }
                _ => {}
            }
        }
    }
}
