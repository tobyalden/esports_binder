use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, GamepadAxis, GamepadButton, Key};

fn main() -> tetra::Result {
    ContextBuilder::new("esports binder", 500, 500)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    content: String,
    vector_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let content = "test".to_string();
        let vector_text = Text::new(
            "default",
            Font::vector(ctx, "./resources/arialbold.ttf", 16.0)?,
        );

        Ok(GameState {
            vector_text,
            content,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.content = input::get_gamepad_name(ctx, 0).unwrap_or("No gamepad detected".to_string());
        self.vector_text.set_content(&self.content);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.vector_text.draw(ctx, Vec2::new(16.0, 16.0));
        Ok(())
    }
}
