use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    Ok(())
}
