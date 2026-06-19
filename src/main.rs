mod app;
mod cursor;
mod event;
mod game;
mod key_input;
mod map;
mod render;
mod state;
mod terminal;

fn main() -> anyhow::Result<()> {
    app::app()?;
    Ok(())
}
