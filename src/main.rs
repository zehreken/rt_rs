mod app;
mod basics;
mod gui;
mod renderer;

fn main() {
    pollster::block_on(app::start());
}
