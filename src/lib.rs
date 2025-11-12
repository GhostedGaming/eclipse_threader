#![no_std]

extern crate alloc;

pub mod runtime;
pub mod scheduler;

pub mod error {
    use core::fmt;
    use alloc::string::String;

    #[derive(Debug)]
    pub enum RuntimeError {
        TaskQueueFull,

        RuntimeNotInitialized,

        TaskPanic(String),
    }

    impl fmt::Display for RuntimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RuntimeError::TaskQueueFull => write!(f, "Task queue is full"),
                RuntimeError::RuntimeNotInitialized => write!(f, "Runtime not initialized"),
                RuntimeError::TaskPanic(msg) => write!(f, "Task panicked: {}", msg),
            }
        }
    }
}