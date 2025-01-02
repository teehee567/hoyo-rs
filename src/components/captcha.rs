use crate::models::auth::geetest::{SessionMMT, SessionMMTResult};


/// Captcha Solver Trait
pub trait CaptchaSolver {
    /// Solve retusn the completed captcha solve data
    fn solve(&self, session: SessionMMT) -> SessionMMTResult;
}

pub(crate) struct DefaultSolver;

impl DefaultSolver {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl CaptchaSolver for DefaultSolver {
    fn solve(&self, session:SessionMMT) -> SessionMMTResult {
        todo!()
    }
}
