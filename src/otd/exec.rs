use std::rc::Rc;

use serde_json::Value;

use super::{
    context::{Context, CtxValue},
    otd::Otd,
};

pub trait IFunc {
    fn get(
        &self,
        name: &str,
    ) -> Option<fn(ctx: Context, otd: &Otd, ifunc: &dyn IFunc) -> Option<(String, Rc<CtxValue>)>>;
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
            let ctx = self.ctx.son();
            if let Some((n, v)) = self.ifunc.get(&otd.func).unwrap()(ctx, otd, self.ifunc.as_ref())
            {
                self.ctx.insert(n, v);
            }
        }
    }
}
