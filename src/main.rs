use crate::args::Args;

mod args;

fn main() -> anyhow::Result<()> {
    let args = Args::new();
    args.run()
}
