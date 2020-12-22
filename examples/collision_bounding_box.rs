use geometry::base::*;
use geometry::collision::*;
use geometry::shape::*;
use ggez::*;

pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);

struct GameState {
    controlled_object: BoundingBox,
    static_box: BoundingBox,
    moving_box: BoundingBox,
    static_point: Point,
    static_circle: Circle,
    hit_static_box: Option<Hit>,
    hit_moving_box: Option<Hit>,
    hit_static_point: Option<Hit>,
    hit_static_circle: Option<Hit>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            controlled_object: BoundingBox::new(Point::zero(), Size::new(50.0, 20.0)),
            static_box: BoundingBox::new(
                Point::new(ORIGIN.0 + 200.0, ORIGIN.1 + 100.0),
                Size::new(100.0, 50.0),
            ),
            moving_box: BoundingBox::new(
                Point::new(ORIGIN.0, ORIGIN.1 - 100.0),
                Size::new(100.0, 50.0),
            ),
            static_point: Point::new(ORIGIN.0 - 300.0, ORIGIN.1 + 100.0),
            static_circle: Circle::new(Point::new(ORIGIN.0, ORIGIN.1 + 200.0), 50.0),
            hit_static_box: None,
            hit_moving_box: None,
            hit_static_point: None,
            hit_static_circle: None,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse = input::mouse::position(ctx);
        self.controlled_object.center.x = mouse.x;
        self.controlled_object.center.y = mouse.y;

        self.moving_box.center.x += 1.0;
        if self.moving_box.center.x >= SCREEN_SIZE.0 {
            self.moving_box.center.x = 0.0
        }

        self.hit_static_box = self.controlled_object.hit_bounding_box(self.static_box);
        self.hit_moving_box = self.controlled_object.hit_bounding_box(self.moving_box);
        self.hit_static_point = self.controlled_object.hit_point(self.static_point);
        self.hit_static_circle = self.controlled_object.hit_circle(&self.static_circle);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        draw_bounding_box(ctx, self.static_box, graphics::WHITE)?;
        draw_bounding_box(ctx, self.moving_box, graphics::WHITE)?;
        draw_circle(ctx, &self.static_circle, graphics::WHITE)?;
        draw_point(ctx, self.static_point, graphics::WHITE)?;
        if let Some(hit) = self.hit_static_box {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjsted = self.controlled_object.clone();
            adjsted.center = adjsted.center + hit.delta;
            draw_bounding_box(ctx, adjsted, [0.0, 1.0, 0.0, 1.0].into())?;
            draw_hit(ctx, hit)?;
        } else if let Some(hit) = self.hit_moving_box {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjsted = self.controlled_object.clone();
            adjsted.center = adjsted.center + hit.delta;
            draw_bounding_box(ctx, adjsted, [0.0, 1.0, 0.0, 1.0].into())?;
            draw_hit(ctx, hit)?;
        } else if let Some(hit) = self.hit_static_point {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjsted = self.controlled_object.clone();
            adjsted.center = adjsted.center + hit.delta;
            draw_bounding_box(ctx, adjsted, [0.0, 1.0, 0.0, 1.0].into())?;
            draw_hit(ctx, hit)?;
        } else if let Some(hit) = self.hit_static_circle {
            draw_bounding_box(ctx, self.controlled_object, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjsted = self.controlled_object.clone();
            adjsted.center = adjsted.center + hit.delta;
            draw_bounding_box(ctx, adjsted, [0.0, 1.0, 0.0, 1.0].into())?;
            draw_hit(ctx, hit)?;
        } else {
            draw_bounding_box(ctx, self.controlled_object, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("test", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("test"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    input::mouse::set_cursor_hidden(ctx, true);
    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}

fn draw_rectangle(ctx: &mut Context, rectangle: &Rectangle, color: graphics::Color) -> GameResult {
    let mut vertices: [mint::Point2<f32>; 4] = [mint::Point2 { x: 0.0, y: 0.0 }; 4];
    for (idx, vertex) in rectangle.polygon().vertices.iter().enumerate() {
        vertices[idx].x = vertex.x;
        vertices[idx].y = vertex.y;
    }
    let mesh = graphics::Mesh::new_polygon(ctx, graphics::DrawMode::stroke(3.0), &vertices, color)?;
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
fn draw_circle(ctx: &mut Context, circle: &Circle, color: graphics::Color) -> GameResult {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::stroke(3.0),
        mint::Point2 {
            x: circle.center().x,
            y: circle.center().y,
        },
        circle.radius(),
        0.1,
        color,
    )?;
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
fn draw_vector(
    ctx: &mut Context,
    origin: Point,
    vector: Vector,
    color: graphics::Color,
) -> GameResult {
    let endpoint = origin + vector;
    let mut points: [mint::Point2<f32>; 2] = [mint::Point2 { x: 0.0, y: 0.0 }; 2];
    points[0].x = origin.x;
    points[0].y = origin.y;
    points[1].x = endpoint.x;
    points[1].y = endpoint.y;
    let mesh = graphics::Mesh::new_line(ctx, &points, 2.0, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    let angle = vector.orientation() + Angle::new(150f64);
    points[0].x = endpoint.x;
    points[0].y = endpoint.y;
    points[1].x = endpoint.x + angle.cos() as f32 * 5.0;
    points[1].y = endpoint.y + angle.sin() as f32 * 5.0;
    let mesh = graphics::Mesh::new_line(ctx, &points, 2.0, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    let angle = vector.orientation() - Angle::new(150f64);
    points[0].x = endpoint.x;
    points[0].y = endpoint.y;
    points[1].x = endpoint.x + angle.cos() as f32 * 5.0;
    points[1].y = endpoint.y + angle.sin() as f32 * 5.0;
    let mesh = graphics::Mesh::new_line(ctx, &points, 2.0, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
fn draw_hit(ctx: &mut Context, hit: Hit) -> GameResult {
    draw_vector(
        ctx,
        hit.contact - hit.delta,
        hit.delta,
        [1.0, 0.0, 1.0, 1.0].into(),
    )?;
    draw_vector(
        ctx,
        hit.contact,
        hit.normal * 15.0,
        [1.0, 1.0, 0.0, 1.0].into(),
    )?;
    draw_point(ctx, hit.contact, [0.0, 1.0, 1.0, 1.0].into())?;
    Ok(())
}
