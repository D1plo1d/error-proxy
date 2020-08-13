use std::error::Error;

#[cfg(backtrace)]
use crate::backtrace::Backtrace;

pub trait ErrorProxy {
    fn source(&self) -> Option<&(dyn Error + 'static)>;

    #[cfg(backtrace)]
    fn backtrace(&self) -> Option<&Backtrace>;
}

impl ErrorProxy for dyn std::error::Error + 'static {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    #[cfg(backtrace)]
    fn backtrace(&self) -> Option<&Backtrace> {
        std::error::Error::backtrace(self)
    }
}

// // [Implementation in anyhow crate]
// impl ErrorProxy for anyhow::Error {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         <self as anyhow::Error>.source()
//     }

//     #[cfg(backtrace)]
//     fn backtrace(&self) -> Option<&Backtrace> {
//         <self as anyhow::Error>::backtrace(self)
//     }
// }

// // [Implementation in chained-error crate]
// impl ErrorProxy for error_chain::ChainedError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         <self as error_chain::ChainedError>.source()
//     }

//     #[cfg(backtrace)]
//     fn backtrace(&self) -> Option<&Backtrace> {
//         <self as error_chain::ChainedError>::backtrace(self)
//     }
// }
