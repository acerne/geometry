use geometry::base::*;
use geometry::collision::*;
use ggez::*;

pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);

struct GameState {
    static_object: BoundingBox,
    moving_object: BoundingBox,
    controlled_object: BoundingBox,
    hit_static: Option<Hit>,
    hit_moving: Option<Hit>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            static_object: BoundingBox::new(
                Point::new(ORIGIN.0, ORIGIN.1 + 100.0),
                Size::new(100.0, 50.0),
            ),
            moving_object: BoundingBox::new(
                Point::new(ORIGIN.0, ORIGIN.1 - 100.0),
                Size::new(100.0, 50.0),
            ),
            controlled_object: BoundingBox::new(Point::zero(), Size::new(20.0, 20.0)),
            hit_static: None,
            hit_moving: None,
        }
    }
}

fn draw_bounding_box(
    ctx: &mut Context,
    bounding_box: BoundingBox,
    color: graphics::Color,
) -> GameResult {
    let rect = graphics::Rect {
        x: bounding_box.x1(),
        y: bounding_box.y1(),
        w: bounding_box.width(),
        h: bounding_box.height(),
    };
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), rect, color)?;
    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
fn draw_point(ctx: &mut Context, point: Point, color: graphics::Color) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        mint::Point2 {
            x: point.x,
            y: point.y,
        },
        2.0,
        0.1,
        color,
    )?;
    graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse = input::mouse::position(ctx);
        self.controlled_object.center.x = mouse.x;
        self.controlled_object.center.y = mouse.y;
        self.moving_object.center.x += 1.0;
        if self.moving_object.center.x >= SCREEN_SIZE.0 {
            self.moving_object.center.x = 0.0
        }

        self.hit_static = self.static_object.hit_bounding_box(self.controlled_object);
        self.hit_moving = self.moving_object.hit_bounding_box(self.controlled_object);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        draw_bounding_box(ctx, self.static_object, graphics::WHITE)?;
        draw_bounding_box(ctx, self.moving_object, graphics::WHITE)?;
        if let Some(hit) = self.hit_static {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut contact = self.controlled_object.clone();
            contact.center = contact.center + hit.delta;
            draw_bounding_box(ctx, contact, [0.0, 1.0, 0.0, 1.0].into())?;
        } else if let Some(hit) = self.hit_moving {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut contact = self.controlled_object.clone();
            contact.center = contact.center + hit.delta;
            draw_bounding_box(ctx, contact, [0.0, 1.0, 0.0, 1.0].into())?;
        } else {
            draw_bounding_box(ctx, self.controlled_object, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("hexpong", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("HexPong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    input::mouse::set_cursor_hidden(ctx, true);
    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
