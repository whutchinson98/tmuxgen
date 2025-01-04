use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// Custom path to the tmuxgen file.
    /// This will override the default lookup path which is the directory the cli was executed in.
    #[arg(short = 'f', long = "tmuxgen-file")]
    pub tmuxgen_file: Option<String>,
}
