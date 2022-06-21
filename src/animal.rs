//use crate::manager::GameManager;
use macroquad::prelude::*;

/// Animal
#[derive(Clone, Debug)]
pub struct Animal {
    pub name: String,
    pub age: i8,
    pub color: Color,
    pub pos: Position,
    pub size: Size,
    pub state: State,
    pub facing: Facing,
    pub texture: Texture2D,
    pub shadow: Texture2D,
}

impl Animal {
    pub fn draw(&self) {
        // SHADOW
        draw_texture_ex(
            self.shadow,
            self.pos.x,
            self.pos.y + self.size.h - 10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.w, self.size.h / 3.0)),
                source: None,
                rotation: 0.0,
                flip_x: match self.facing {
                    Facing::Left => false,
                    Facing::Right => true,
                },
                flip_y: false,
                pivot: None,
            },
        );

        // BODY
        if let State::Selected = self.state {
            draw_texture_ex(
                self.texture,
                self.pos.x,
                self.pos.y,
                self.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size.w, self.size.h)),
                    source: None,
                    rotation: 20.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        } else {
            draw_texture_ex(
                self.texture,
                self.pos.x,
                self.pos.y,
                self.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size.w, self.size.h)),
                    source: None,
                    rotation: 0.0,
                    flip_x: match self.facing {
                        Facing::Left => false,
                        Facing::Right => true,
                    },
                    flip_y: false,
                    pivot: None,
                },
            );
        }

        // Debug state
        if true {
            //Debug state
            draw_text(
                match self.state {
                    State::Walking {
                        from: _,
                        target: _,
                        speed: _,
                    } => "walking",
                    State::Rest(_) => "rest",
                    State::Selected => "selected",
                },
                self.pos.x,
                self.pos.y,
                18.0,
                WHITE,
            );

            draw_text(&self.name, self.pos.x, self.pos.y - 20.0, 18.0, WHITE);

            // TARGET
            if let State::Walking {
                from: _,
                target,
                speed: _,
            } = self.state
            {
                draw_rectangle_lines(target.x, target.y, self.size.w, self.size.h, 1.0, BLUE);
            }
        }
    }

    pub fn update(&mut self) {
        self.draw();

        match self.state {
            State::Walking {
                from,
                target,
                speed,
            } => {
                if target == self.pos {
                    self.switch_state()
                } else {
                    let movement = speed.get_speed();
                    self.pos.x += (target.x - from.x) / movement;
                    self.pos.y += (target.y - from.y) / movement;
                }
            }
            State::Rest(until) => {
                if get_time() >= until {
                    self.switch_state();
                }
            }
            State::Selected => {
                if is_mouse_button_released(MouseButton::Left) {
                    self.switch_state();
                    return;
                }

                self.pos.x = mouse_position().0;
                self.pos.y = mouse_position().1;
            }
        }
    }

    fn switch_state(&mut self) {
        match self.state {
            State::Walking {
                from: _,
                target: _,
                speed: _,
            } => self.state = State::Rest(get_time() + rand::gen_range(1.0, 3.0)),
            State::Rest(_) => {
                let new_x = rand::gen_range(200.0, screen_width() - 200.0);
                let new_y = rand::gen_range(200.0, screen_height() - 200.0);

                if new_x > self.pos.x {
                    self.facing = Facing::Right;
                } else {
                    self.facing = Facing::Left;
                }

                self.state = State::Walking {
                    from: self.pos,
                    target: Position { x: new_x, y: new_y },
                    speed: Speed::new(rand::gen_range(0.2, 1.0)),
                }
            }
            State::Selected => self.state = State::Rest(get_time() + rand::gen_range(1.0, 3.0)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Walking {
        from: Position,
        target: Position,
        speed: Speed,
    },
    Rest(f64),
    Selected,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.x > other.x - 0.5 && self.x < other.x + 0.5)
            && (self.y > other.y - 0.5 && self.y < other.y + 0.5)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Speed(f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(1000.0 / speed)
    }

    pub fn get_speed(&self) -> f32 {
        self.0
    }
}
