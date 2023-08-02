use game_engine::*;
fn main() {
  let mut x_position = 200.;
  let mut y_position = 30.;

  let mut x_direction = 1.;
  let mut y_direction = 1.;

  let speed = 5.;
  set_event_handler(move |event| { 
    match event {
      Event::Draw => {
        x_position += x_direction * speed;
        y_position += y_direction * speed;

        if x_position < 0. || x_position > 1000. {
          x_direction *= -1.;
        }
        if y_position < 0. || y_position > 700. {
          y_direction *= -1.;
        }

        clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        draw_rectangle(x_position, y_position, 100., 100.);
      }
      _ => {}
    }

    clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
    draw_rectangle(x_position, y_position, 100., 100.);
  }) 
}