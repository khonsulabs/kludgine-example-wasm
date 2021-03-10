extern crate kludgine;
use kludgine::prelude::*;
use log::info;
use wasm_bindgen::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(web_sys_unstable_apis)]
    info!("Unstable apis enabled");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    SingleWindowApplication::run(Simple::default());
}

#[derive(Default)]
struct Simple {
    source_sprite: Option<SpriteSource>,
    rotation_angle: Angle,
}

impl WindowCreator for Simple {
    fn window_title() -> String {
        "Simple - Kludgine".to_owned()
    }
}

impl Window for Simple {
    fn target_fps(&self) -> Option<u16> {
        Some(60)
    }
}

impl StandaloneComponent for Simple {}

#[async_trait]
impl Component for Simple {
    async fn initialize(&mut self, _context: &mut Context) -> KludgineResult<()> {
        let texture = kludgine::include_texture!("../k.png")?;
        self.source_sprite = Some(SpriteSource::entire_texture(texture));
        Ok(())
    }

    async fn update(&mut self, context: &mut Context) -> KludgineResult<()> {
        if let Some(elapsed) = context.scene().elapsed().await {
            self.rotation_angle += Angle::radians(elapsed.as_secs_f32());
        }

        Ok(())
    }

    async fn render(&mut self, context: &mut StyledContext, layout: &Layout) -> KludgineResult<()> {
        let sprite = self.source_sprite.as_ref().unwrap();

        sprite
            .render_at(
                context.scene(),
                layout.bounds_without_margin().center(),
                SpriteRotation::around_center(self.rotation_angle),
            )
            .await;

        Ok(())
    }
}
