use super::error::ImapResult;
mod flags;
mod response;
mod scanner;

pub use scanner::{Scan, Scanner};

pub trait Response: Scan
where
    Self: Sized,
{
    fn convert(s: &str) -> Self {
        let scanner = Scanner;

        Self::scan(s, scanner)
    }
    fn is_err(&self) -> ImapResult<()>;
}

mod authenticated;
mod not_authenticated;

pub use authenticated::*;
pub use not_authenticated::*;
