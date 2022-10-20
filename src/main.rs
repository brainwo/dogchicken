pub mod animal;
pub mod manager;
pub mod name;

use animal::{Animal, Pet, Position, Size};
use itertools::Itertools;
use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::ui::*;
use manager::GameManager;
use name::NameGenerator;

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let body_colors = vec![RED, YELLOW, GREEN, BLUE];

    let name = NameGenerator::generate();
    let pet_name = NameGenerator::generate();

    let start_position = Position {
        x: screen_width() / 2.0,
        y: 100.0,
    };

    let texture =
        Texture2D::from_file_with_format(include_bytes!("../resources/chickin.png"), None);
    let shadow = Texture2D::from_file_with_format(include_bytes!("../resources/shadow.png"), None);

    let template = Animal {
        name,
        color: RED,
        pos: start_position,
        size: Size { w: 50.0, h: 50.0 },
        state: animal::State::Rest(get_time()),
        facing: animal::Facing::Left,
        texture,
        shadow,
        age: 10,
        pet: Some(Pet {
            name: pet_name,
            shadow,
            texture,
        }),
    };

    let mut animal_vec: Vec<Animal> = match storage::try_get_mut::<GameManager>() {
        Some(manager) => manager.animals.clone(),
        None => {
            let new_animals = vec![template.clone()];
            storage::store(GameManager {
                animals: new_animals.clone(),
                debug: true,
            });
            new_animals
        }
    };

    loop {
        clear_background(DARKGRAY);

        for (i, item) in animal_vec.clone().iter().enumerate() {
            if root_ui().button(None, item.name.clone()) {
                animal_vec.remove(i);
            }
        }

        if (is_key_pressed(macroquad::input::KeyCode::Space) || root_ui().button(None, "Add"))
            && animal_vec.len() < 20
        {
            let start_position = Position {
                x: rand::gen_range(100.0, screen_width() - 100.0),
                y: rand::gen_range(100.0, screen_height() - 100.0),
            };

            animal_vec.push(Animal {
                name: NameGenerator::generate(),
                pos: start_position,
                size: Size {
                    w: rand::gen_range(45.0, 60.0),
                    h: rand::gen_range(45.0, 60.0),
                },
                color: *body_colors.choose().unwrap(),
                state: animal::State::Rest(get_time()),
                pet: None,
                ..template
            });

            // 💾 Save data to storage
            storage::store(GameManager {
                animals: animal_vec.clone(),
                debug: true,
            });
            debug!("saved!");
        }

        // Draw everything else
        // Resources full approach
        // TODO: improve this so it's only reorder when it's necessary
        animal_vec
            .iter_mut()
            .sorted_by(|a, b| Ord::cmp(&(a.pos.y as i32), &(b.pos.y as i32)))
            .for_each(|animal| {
                animal.update();
            });

        draw_text(
            format!("Animals count: {}/20", animal_vec.len()).as_str(),
            10.0,
            screen_height() - 30.0,
            30.0,
            WHITE,
        );

        draw_text(
            format!("{}, {}", mouse_position().0, mouse_position().1,).as_str(),
            20.0,
            20.0,
            30.0,
            WHITE,
        );

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            screen_width() - 100.0,
            20.0,
            30.0,
            WHITE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            draw_text(
                "Clicked",
                mouse_position().0,
                mouse_position().1,
                20.0,
                WHITE,
            )
        }

        next_frame().await
    }
}
