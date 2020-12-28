use geometry::base::*;
use geometry::collision::*;
use geometry::shape::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::mouse::*;
use ggez::*;

pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
pub const SMALL_POS: Point = Point {
    x: ORIGIN.0 + 200.0,
    y: ORIGIN.1 + 150.0,
};
pub const LARGE_POS: Point = Point {
    x: ORIGIN.0 - 200.0,
    y: ORIGIN.1 + 150.0,
};
pub const ROTATING_POS: Point = Point {
    x: ORIGIN.0 - 200.0,
    y: ORIGIN.1 - 150.0,
};
pub const MOVING_POS: Point = Point {
    x: ORIGIN.0 + 200.0,
    y: ORIGIN.1 - 150.0,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectShape {
    Point,
    BoundingBox,
    Circle,
    Ray,
}

pub struct Object {
    circle: Circle,
    point: Point,
    bounding_box: BoundingBox,
    ray: Ray,
}
impl Object {
    fn default() -> Self {
        Self {
            circle: Circle::new(Point::zero(), 0.0),
            point: Point::zero(),
            bounding_box: BoundingBox::new(Point::zero(), Size::zero()),
            ray: Ray::new(Point::zero(), Angle::zero(), 0.0),
        }
    }
    fn rotate(&mut self, angle: Angle) {
        self.ray.direction = self.ray.direction + angle;
    }
    fn translate(&mut self, translation: Vector) {
        self.point = self.point + translation;
        self.circle.move_to(self.point);
        self.bounding_box.center = self.point;
        self.ray.origin = self.point;
    }
    fn move_to(&mut self, location: Point) {
        self.point = location;
        self.circle.move_to(location);
        self.bounding_box.center = location;
        self.ray.origin = location;
    }
    fn hit(
        &self,
        self_shape: ObjectShape,
        other: &Object,
        other_shape: ObjectShape,
    ) -> Option<Hit> {
        match self_shape {
            ObjectShape::BoundingBox => match other_shape {
                ObjectShape::Point => {
                    return self.bounding_box.hit_point(other.point);
                }
                ObjectShape::Circle => {
                    return self.bounding_box.hit_circle(&other.circle);
                }
                ObjectShape::BoundingBox => {
                    return self.bounding_box.hit_bounding_box(other.bounding_box);
                }
                ObjectShape::Ray => {
                    return None;
                }
            },
            ObjectShape::Ray => match other_shape {
                ObjectShape::Point => {
                    return self.ray.hit_point(other.point);
                }
                ObjectShape::Circle => {
                    return self.ray.hit_circle(&other.circle);
                }
                ObjectShape::BoundingBox => {
                    return self.ray.hit_bounding_box(other.bounding_box);
                }
                ObjectShape::Ray => {
                    return None;
                }
            },
            ObjectShape::Circle => match other_shape {
                ObjectShape::Point => {
                    return self.circle.hit_point(other.point);
                }
                ObjectShape::Circle => {
                    return self.circle.hit_circle(&other.circle);
                }
                ObjectShape::BoundingBox => {
                    return self.circle.hit_bounding_box(other.bounding_box);
                }
                ObjectShape::Ray => {
                    return None;
                }
            },
            _ => None,
        }
    }
}

struct GameState {
    controlled_object: Object,
    controlled_shape: ObjectShape,
    object_shape: ObjectShape,
    small_object: Object,
    large_object: Object,
    rotating_object: Object,
    moving_object: Object,
    hit_small: Option<Hit>,
    hit_large: Option<Hit>,
    hit_rotating: Option<Hit>,
    hit_moving: Option<Hit>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            controlled_object: Object::default(),
            controlled_shape: ObjectShape::BoundingBox,
            object_shape: ObjectShape::BoundingBox,
            small_object: Object::default(),
            large_object: Object::default(),
            rotating_object: Object::default(),
            moving_object: Object::default(),
            hit_small: None,
            hit_large: None,
            hit_rotating: None,
            hit_moving: None,
        }
    }
    pub fn respawn(&mut self) {
        self.controlled_object.point = Point::zero();
        self.small_object.point = SMALL_POS.clone();
        self.large_object.point = LARGE_POS.clone();
        self.rotating_object.point = ROTATING_POS.clone();
        self.moving_object.point = MOVING_POS.clone();

        self.controlled_object.circle = Circle::new(Point::zero(), 50.0);
        self.small_object.circle = Circle::new(SMALL_POS, 20.0);
        self.large_object.circle = Circle::new(LARGE_POS, 100.0);
        self.rotating_object.circle = Circle::new(ROTATING_POS, 50.0);
        self.moving_object.circle = Circle::new(MOVING_POS, 50.0);

        self.controlled_object.bounding_box =
            BoundingBox::new(Point::zero(), Size::new(50.0, 20.0));
        self.small_object.bounding_box = BoundingBox::new(SMALL_POS, Size::new(20.0, 10.0));
        self.large_object.bounding_box = BoundingBox::new(LARGE_POS, Size::new(100.0, 30.0));
        self.rotating_object.bounding_box = BoundingBox::new(ROTATING_POS, Size::new(50.0, 50.0));
        self.moving_object.bounding_box = BoundingBox::new(MOVING_POS, Size::new(50.0, 50.0));

        self.controlled_object.ray = Ray::new(Point::zero(), Angle::zero(), 50.0);
        self.small_object.ray = Ray::new(SMALL_POS, Angle::zero(), 20.0);
        self.large_object.ray = Ray::new(LARGE_POS, Angle::new(180.0), 100.0);
        self.rotating_object.ray = Ray::new(ROTATING_POS, Angle::zero(), 50.0);
        self.moving_object.ray = Ray::new(MOVING_POS, Angle::zero(), 50.0);
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse = input::mouse::position(ctx);
        self.controlled_object.move_to(Point::new(mouse.x, mouse.y));

        self.rotating_object.rotate(Angle::new(1.0));
        self.moving_object.translate(Vector::new(1.0, 0.0));
        if self.moving_object.point.x > SCREEN_SIZE.0 {
            self.moving_object
                .translate(Vector::new(-SCREEN_SIZE.0 / 2.0, 0.0));
        }

        self.hit_small = self.controlled_object.hit(
            self.controlled_shape,
            &self.small_object,
            self.object_shape,
        );
        self.hit_large = self.controlled_object.hit(
            self.controlled_shape,
            &self.large_object,
            self.object_shape,
        );
        self.hit_rotating = self.controlled_object.hit(
            self.controlled_shape,
            &self.rotating_object,
            self.object_shape,
        );
        self.hit_moving = self.controlled_object.hit(
            self.controlled_shape,
            &self.moving_object,
            self.object_shape,
        );

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        draw_object(ctx, &self.small_object, self.object_shape, graphics::WHITE)?;
        draw_object(ctx, &self.large_object, self.object_shape, graphics::WHITE)?;
        draw_object(
            ctx,
            &self.rotating_object,
            self.object_shape,
            graphics::WHITE,
        )?;
        draw_object(ctx, &self.moving_object, self.object_shape, graphics::WHITE)?;

        if let Some(hit) = self.hit_small {
            draw_hit_object(ctx, &self.controlled_object, self.controlled_shape, hit);
        } else if let Some(hit) = self.hit_large {
            draw_hit_object(ctx, &self.controlled_object, self.controlled_shape, hit);
        } else if let Some(hit) = self.hit_rotating {
            draw_hit_object(ctx, &self.controlled_object, self.controlled_shape, hit);
        } else if let Some(hit) = self.hit_moving {
            draw_hit_object(ctx, &self.controlled_object, self.controlled_shape, hit);
        } else {
            draw_object(
                ctx,
                &self.controlled_object,
                self.controlled_shape,
                [0.0, 0.0, 1.0, 1.0].into(),
            );
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Q => {
                match self.object_shape {
                    ObjectShape::Point => {
                        self.object_shape = ObjectShape::Circle;
                    }
                    ObjectShape::Circle => {
                        self.object_shape = ObjectShape::BoundingBox;
                    }
                    ObjectShape::BoundingBox => {
                        self.object_shape = ObjectShape::Ray;
                    }
                    ObjectShape::Ray => {
                        self.object_shape = ObjectShape::Point;
                    }
                }
                self.respawn();
            }
            KeyCode::W => {
                match self.controlled_shape {
                    ObjectShape::Point => {
                        self.controlled_shape = ObjectShape::Circle;
                    }
                    ObjectShape::Circle => {
                        self.controlled_shape = ObjectShape::BoundingBox;
                    }
                    ObjectShape::BoundingBox => {
                        self.controlled_shape = ObjectShape::Ray;
                    }
                    ObjectShape::Ray => {
                        self.controlled_shape = ObjectShape::Point;
                    }
                }
                self.respawn();
            }
            _ => (),
        }
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.controlled_object.rotate(Angle::new(30f64));
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("test", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("test"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    input::mouse::set_cursor_hidden(ctx, true);
    let state = &mut GameState::new();
    state.respawn();
    event::run(ctx, events_loop, state)
}

pub fn draw_rectangle(
    ctx: &mut Context,
    rectangle: &Rectangle,
    color: graphics::Color,
) -> GameResult {
    let mut vertices: [mint::Point2<f32>; 4] = [mint::Point2 { x: 0.0, y: 0.0 }; 4];
    for (idx, vertex) in rectangle.polygon().vertices.iter().enumerate() {
        vertices[idx].x = vertex.x;
        vertices[idx].y = vertex.y;
    }
    let mesh = graphics::Mesh::new_polygon(ctx, graphics::DrawMode::stroke(3.0), &vertices, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
pub fn draw_bounding_box(
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
pub fn draw_circle(ctx: &mut Context, circle: &Circle, color: graphics::Color) -> GameResult {
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
pub fn draw_point(ctx: &mut Context, point: Point, color: graphics::Color) -> GameResult {
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
pub fn draw_ray(ctx: &mut Context, ray: Ray, color: graphics::Color) -> GameResult {
    let mut points: [mint::Point2<f32>; 2] = [mint::Point2 { x: 0.0, y: 0.0 }; 2];
    points[0].x = ray.origin.x;
    points[0].y = ray.origin.y;
    points[1].x = ray.origin.x + ray.vector().dx;
    points[1].y = ray.origin.y + ray.vector().dy;
    let mesh = graphics::Mesh::new_line(ctx, &points, 2.0, color)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
pub fn draw_vector(
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
pub fn draw_hit(ctx: &mut Context, hit: Hit) -> GameResult {
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
pub fn draw_hit_object(
    ctx: &mut Context,
    object: &Object,
    shape: ObjectShape,
    hit: Hit,
) -> GameResult {
    match shape {
        ObjectShape::Point => {
            draw_point(ctx, object.point, [1.0, 0.0, 0.0, 1.0].into())?;
            draw_point(ctx, object.point + hit.delta, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        ObjectShape::Circle => {
            draw_circle(ctx, &object.circle, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjusted = object.circle.clone();
            adjusted.translate(hit.delta);
            draw_circle(ctx, &adjusted, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        ObjectShape::BoundingBox => {
            draw_bounding_box(ctx, object.bounding_box, [1.0, 0.0, 0.0, 1.0].into())?;
            let mut adjusted = object.bounding_box.clone();
            adjusted.center = adjusted.center + hit.delta;
            draw_bounding_box(ctx, adjusted, [0.0, 1.0, 0.0, 1.0].into())?;
        }
        ObjectShape::Ray => {
            draw_ray(ctx, object.ray, [1.0, 0.0, 0.0, 1.0].into())?;
            if hit.time > 0.0 {
                let mut hit_ray = object.ray.clone();
                hit_ray.length = hit_ray.length * hit.time;
                draw_ray(ctx, hit_ray, [0.0, 1.0, 0.0, 1.0].into())?;
            }
        }
    }
    draw_hit(ctx, hit)?;
    Ok(())
}
pub fn draw_object(
    ctx: &mut Context,
    object: &Object,
    shape: ObjectShape,
    color: graphics::Color,
) -> GameResult {
    match shape {
        ObjectShape::Point => {
            draw_point(ctx, object.point, color)?;
        }
        ObjectShape::Circle => {
            draw_circle(ctx, &object.circle, color)?;
        }
        ObjectShape::BoundingBox => {
            draw_bounding_box(ctx, object.bounding_box, color)?;
        }
        ObjectShape::Ray => {
            draw_ray(ctx, object.ray, color)?;
        }
    }
    Ok(())
}
