use geometry::base::*;
use geometry::collision::*;
use geometry::shape::*;
use ggez::input::mouse::*;
use ggez::*;

pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);

struct GameState {
    static_object: BoundingBox,
    moving_object: BoundingBox,
    controlled_object: Ray,
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
            controlled_object: Ray::new(Point::zero(), Angle::zero(), 200.0),
            hit_static: None,
            hit_moving: None,
        }
    }
}

fn draw_rectangle(ctx: &mut Context, rectangle: &Rectangle, color: graphics::Color) -> GameResult {
    let mut vertices: [mint::Point2<f32>; 4] = [mint::Point2 { x: 0.0, y: 0.0 }; 4];
    for (idx, vertex) in rectangle.polygon().vertices.iter().enumerate() {
        vertices[idx].x = vertex.x;
        vertices[idx].y = vertex.y;
    }
    let mesh = graphics::Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &vertices, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
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
    let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), rect, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
fn draw_point(ctx: &mut Context, point: Point, color: graphics::Color) -> GameResult {
    let mesh = graphics::Mesh::new_circle(
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
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
fn draw_ray(ctx: &mut Context, ray: Ray, color: graphics::Color) -> GameResult {
    let mut points: [mint::Point2<f32>; 2] = [mint::Point2 { x: 0.0, y: 0.0 }; 2];
    points[0].x = ray.origin.x;
    points[0].y = ray.origin.y;
    points[1].x = ray.origin.x + ray.vector().dx;
    points[1].y = ray.origin.y + ray.vector().dy;
    let mesh = graphics::Mesh::new_line(ctx, &points, 2.0, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse = input::mouse::position(ctx);
        self.controlled_object.origin.x = mouse.x;
        self.controlled_object.origin.y = mouse.y;
        self.moving_object.center.x += 1.0;
        if self.moving_object.center.x >= SCREEN_SIZE.0 {
            self.moving_object.center.x = 0.0
        }

        self.hit_static = self.controlled_object.hit_bounding_box(self.static_object);
        self.hit_moving = self.controlled_object.hit_bounding_box(self.moving_object);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        draw_bounding_box(ctx, self.static_object, graphics::WHITE)?;
        draw_bounding_box(ctx, self.moving_object, graphics::WHITE)?;
        if let Some(hit) = self.hit_static {
            draw_ray(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            if hit.time > 0.0 {
                let mut hit_ray = self.controlled_object.clone();
                hit_ray.length = hit_ray.length * hit.time;
                draw_ray(ctx, hit_ray, [0.0, 1.0, 0.0, 1.0].into())?;
            }
        } else if let Some(hit) = self.hit_moving {
            draw_ray(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            if hit.time > 0.0 {
                let mut hit_ray = self.controlled_object.clone();
                hit_ray.length = hit_ray.length * hit.time;
                draw_ray(ctx, hit_ray, [0.0, 1.0, 0.0, 1.0].into())?;
            }
        } else {
            draw_ray(ctx, self.controlled_object, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.controlled_object.direction = Angle::new(self.controlled_object.direction.deg + 30f64);
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
