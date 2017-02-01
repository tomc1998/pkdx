#[macro_use]
extern crate guitk;

use guitk::logger;

fn setup_logger() {
  logger::set_default_log_tag("pkdx");
  logger::set_default_log_priority(logger::LogPriority::DEBUG);
}

fn setup_test_layer(w: f32, h: f32) -> guitk::view::Layer {
  use guitk::view::*;
  use guitk::entity::core::*;
  use guitk::common::color::*;
  use guitk::layout::*;
  use guitk::entity::EntityID;
  // Header bar
  let header_bar_aabb = ComponentAABB {
    entity_id: EntityID(0), x: 0.0, y: 0.0, w: 0.0, h: 0.0, };
  let header_bar_debug_draw = ComponentDebugDraw {
    entity_id: EntityID(0), color: RGBf32::new(0.0, 1.0, 0.0), };
  // Body container ( nested layer )
  let body_aabb = ComponentAABB {
    entity_id: EntityID(1), x:0.0, y: 0.0, w: 0.0, h: 0.0, };
  let mut body_layer = Layer::new();
  body_layer.entity_id = Some(EntityID(1));

  // Body L div
  let body_l_aabb = ComponentAABB {
    entity_id: EntityID(3), x: 0.0, y: 0.0, w: 0.0, h: 0.0, };
  let body_l_debug_draw = ComponentDebugDraw {
    entity_id: EntityID(3), color: RGBf32::new(1.0, 1.0, 0.0), };
  // Body R div
  let body_r_aabb = ComponentAABB {
    entity_id: EntityID(4), x: 0.0, y: 0.0, w: 0.0, h: 0.0, };
  let body_r_debug_draw = ComponentDebugDraw {
    entity_id: EntityID(4), color: RGBf32::new(0.0, 1.0, 1.0), };

  const SIDEBAR_SIZE: f32 = 300.0;
  // Body vsplit
  let inner_body_aabb = ComponentAABB {
    entity_id: EntityID(5), x: -SIDEBAR_SIZE, y: 0.0, 
    w: w+SIDEBAR_SIZE, h: h - 100.0, };

  let inner_body_container = ComponentContainer {
    entity_id: EntityID(5),
    layout: Layout::VSplit {
      entity_l: EntityID(3),
      entity_r: EntityID(4),
      split_pos: SIDEBAR_SIZE,
    },
  };

  let inner_body_trigger = ComponentTrigger {
    entity_id: EntityID(5),
    trigger_id: 0,
    x: SIDEBAR_SIZE - 30.0, y: 0.0, w: 60.0, h: h - 100.0,
    relative: true,};

  let inner_body_scroll = ComponentTouchScroll {
    entity_id: EntityID(5),
    trigger_id: 0,
    behaviour_flags: scroll_behaviour::LOCKED_Y, 
    min_x:   - SIDEBAR_SIZE,
    max_x: w + SIDEBAR_SIZE,
    min_y: 0.0,
    max_y: h,

  };

  // Main container
  let view_container_aabb = ComponentAABB {
    entity_id: EntityID(2), x: 0.0, y: 0.0, w: w, h: h};
  let view_container_container = ComponentContainer {
    entity_id: EntityID(2), 
    layout: Layout::HeaderBar {
      entity_header: EntityID(0),
      entity_body: EntityID(1),
      header_height: 100.0,
    }
  };

  let mut layer = guitk::view::Layer::new();
  layer.component_aabb.push(header_bar_aabb);
  layer.component_aabb.push(body_aabb);
  layer.component_aabb.push(view_container_aabb);

  layer.component_debug_draw.push(header_bar_debug_draw);

  layer.component_container.push(view_container_container);

  body_layer.component_aabb.push(body_l_aabb);
  body_layer.component_aabb.push(body_r_aabb);
  body_layer.component_aabb.push(inner_body_aabb);

  body_layer.component_debug_draw.push(body_l_debug_draw);
  body_layer.component_debug_draw.push(body_r_debug_draw);

  body_layer.component_container.push(inner_body_container);

  body_layer.component_trigger.push(inner_body_trigger);

  body_layer.component_touch_scroll.push(inner_body_scroll);

  layer.component_layer.push(body_layer);

  return layer;
}

fn setup_test_view<'a>(w: f32, h: f32) -> guitk::view::View<'a> {
  let mut view = guitk::view::View::new();
  view.layers.push(setup_test_layer(w, h));
  return view;
}

fn main() {
  // Initialise guitk
  let mut guitk_state = guitk::init().unwrap();
  setup_logger();

  // Push a new view onto the view stack
  let (w, h) = guitk_state.get_view_size();
  guitk_state.view_stack.push(setup_test_view(w as f32, h as f32));

  // Layout test view
  guitk_state.view_stack.last_mut().unwrap().layout();

  loop {
    // Render the view
    guitk_state.update();
  }
}
