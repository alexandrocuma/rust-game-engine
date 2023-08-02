use game_engine::*;
fn main() {
  let mut x_position = 200.;
  let mut y_position = 30.;

  let mut x_direction = 1.;
  let mut y_direction = 1.;

  let mut speed = 5.;
  set_event_handler(move |context, event| { 
    match event {
      Event::Draw => {
        x_position += x_direction * speed;
        y_position += y_direction * speed;

        if x_position < 0. || x_position > 1350. {
          x_direction *= -1.;
        }
        if y_position < 0. || y_position > 700. {
          y_direction *= -1.;
        }

        context.clear_screen_to_color(0., 0., 0.3, 1.);
        context.draw_rectangle(x_position, y_position, 100., 100., 1., 0., 0., 1.);
      }
      Event::KeyDown(key) => {
        match key {
          Key::Right => x_position += 10.0,
          Key::Left => x_position -= 10.0,
          Key::Up => y_position += 10.0,
          Key::Down => y_position -= 10.0,
          Key::Space => speed = 0. 
        }
      }
    }
  }) 
}