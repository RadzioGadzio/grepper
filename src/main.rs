use crate::args::Args;

mod args;
mod tests;

fn main() -> anyhow::Result<()> {
    let args = Args::new();
    args.run()
}
