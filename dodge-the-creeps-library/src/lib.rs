use gdnative::prelude::*;

pub mod hud;
pub mod main_scene;
pub mod mob;
pub mod player;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<mob::Mob>();
    handle.add_class::<player::Player>();
    handle.add_class::<hud::HUD>();
    handle.add_class::<main_scene::Main>();
}

// Macro that creates the entry-points of the dynamic library
godot_init!(init);
