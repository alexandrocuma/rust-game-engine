use game_engine::*;
fn main() {
  let mut x_position = 200.0;
  let mut y_position = 30.0;

  set_event_handler(move |key| { 
    let move_amount = 20.0;
    match key {
      Key::Left => clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
      Key::Right => clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
      Key::Up => clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
      Key::Down => clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
      Key::Space => clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
    }

    clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
    draw_rectangle(x_position, y_position, 100., 100.);
  }) 
}