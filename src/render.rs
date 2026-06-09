use crossterm::{queue, style::Print};

pub fn render() {
    queue!(Print("hello there!"));
}
