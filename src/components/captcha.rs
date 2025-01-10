use crate::models::auth::geetest::{SessionMMT, SessionMMTResult};

/// Captcha Solver Trait
pub trait CaptchaSolver {
    /// Solve retusn the completed captcha solve data
    fn solve(&self, session: SessionMMT) -> SessionMMTResult;
}

/// Trait to get OTP
pub trait OTPSource {
    /// Gets the OTP for Mobile Login
    fn get_otp(&self) -> String;
}

pub(crate) struct DefaultSolver;

impl DefaultSolver {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl CaptchaSolver for DefaultSolver {
    fn solve(&self, session: SessionMMT) -> SessionMMTResult {
        todo!()
    }
}

impl OTPSource for DefaultSolver {
    fn get_otp(&self) -> String {
        todo!()
    }
}
