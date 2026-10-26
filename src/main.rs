use std::process::ExitCode;
use crate::searcher::{run};

mod args;
mod tests;
mod searcher;
mod matcher;


fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) if is_broken_pipe(&e) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("grepper: {e:#}"); ExitCode::from(2) },
    }
}

fn is_broken_pipe(e: &anyhow::Error) -> bool {
    e.downcast_ref::<std::io::Error>()
        .is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe)
}

