use macroquad::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum BoxState {
    Grow,
    Shrink,
    Stasis,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Square Breath".to_owned(),
        fullscreen: false,
        window_height: 500,
        window_width: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("{:?}", screen_height());
    println!("{:?}", screen_width());
    println!("{:?}", get_fps());

    let radius =
        f32::sqrt((screen_width() * screen_width()) + (screen_height() * screen_height())) / 2.0;

    let minimum_radius = radius * 0.2;
    let maximum_radius = radius * 0.8;

    let mut radius = minimum_radius;
    let mut state: BoxState = BoxState::Grow;
    let seconds_per_movement = 8.0;
    let mut frame_counter = 0.0;

    loop {
        clear_background(BLACK);

        if state == BoxState::Grow {
            radius += 5.8 / seconds_per_movement;
        } else if state == BoxState::Shrink {
            radius -= 5.8 / seconds_per_movement;
        } else if state == BoxState::Stasis {
            radius = radius
        }

        let enter_stasis = frame_counter < get_fps() as f32 * seconds_per_movement;

        if radius < minimum_radius {
            if enter_stasis {
                state = BoxState::Stasis;
                frame_counter += 1.0;
            } else {
                state = BoxState::Grow;
                frame_counter = 0.0;
            }
        } else if radius > maximum_radius {
            if enter_stasis {
                state = BoxState::Stasis;
                frame_counter += 1.0;
            } else {
                state = BoxState::Shrink;
                frame_counter = 0.0;
            }
        }

        draw_poly(
            get_x_percentage(50.0),
            get_y_percentage(50.0),
            4,
            radius,
            45.0,
            GOLD,
        );

        next_frame().await;
    }
}

fn get_x_percentage(percentage: f32) -> f32 {
    screen_width() * percentage / 100.0
}

fn get_y_percentage(percentage: f32) -> f32 {
    screen_height() * percentage / 100.0
}
