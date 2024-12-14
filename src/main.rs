use enigo::*;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();

    draw_smiley(&mut enigo);
}

fn draw_smiley(enigo: &mut Enigo) {
    // left eye
    draw_path(enigo, vec![(510, 200), (510, 250)]);

    // right eye
    draw_path(enigo, vec![(535, 200), (535, 250)]);

    // slasher smile
    let coordinates = vec![
        (490, 260),
        (500, 267),
        (510, 270),
        (520, 275),
        (530, 270),
        (540, 267),
        (550, 260),
    ];
    draw_path(enigo, coordinates);
}

fn draw_path(enigo: &mut Enigo, path: Vec<(i32, i32)>) {
    let mut mouse_pressed = false;
    for coordinate in path {
        sleep(Duration::from_millis(200));
        enigo
            .move_mouse(coordinate.0, coordinate.1, Coordinate::Abs)
            .unwrap();

        if !mouse_pressed {
            sleep(Duration::from_millis(200));
            enigo.button(Button::Left, Direction::Press).unwrap();
            mouse_pressed = true;
        }
    }
    sleep(Duration::from_millis(200));
    enigo.button(Button::Left, Direction::Release).unwrap();
}
