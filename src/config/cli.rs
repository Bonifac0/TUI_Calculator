use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tui_calculator")]
#[command(author = "TUI Calculator Team")]
#[command(version = "0.1.0")]
#[command(about = "Advanced CASIO-style Scientific TUI Calculator with 2D Math & LaTeX Support", long_about = None)]
pub struct Cli {
    /// Force Small layout mode
    #[arg(short = 's', long = "small")]
    pub small: bool,

    /// Force Medium layout mode
    #[arg(short = 'm', long = "medium")]
    pub medium: bool,

    /// Force Big layout mode
    #[arg(short = 'b', long = "big")]
    pub big: bool,

    /// Non-interactive expression evaluation (e.g. "3 * sin(30deg)")
    #[arg(value_name = "EXPRESSION")]
    pub expression: Option<String>,
}
