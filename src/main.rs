#[macro_use]
extern crate guitk;

use guitk::logger;

fn setup_logger() {
  logger::set_default_log_tag("pkdx");
  logger::set_default_log_priority(logger::LogPriority::DEBUG);
}

fn main() {
  // Initialise guitk
  let mut guitk_state = guitk::init();
  setup_logger();

  // Push a new view onto the view stack
  guitk_state.view_stack.push(guitk::view::View::new());

  loop {
    // Render the view
    guitk_state.render();
  }
}
