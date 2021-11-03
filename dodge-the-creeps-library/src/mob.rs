use gdnative::prelude::*;
use gdnative_bindings_custom::prelude::*;
use gdnative_bindings_custom::{AnimatedSprite, RigidBody2D};
use rand::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
#[user_data(user_data::MutexData<Mob>)]
pub struct Mob {
    #[property(default = 150.0)]
    pub min_speed: f32,
    #[property(default = 250.0)]
    pub max_speed: f32,
}

impl Mob {
    /// The "constructor" of the class.
    fn new(_owner: &RigidBody2D) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MobAnimation {
    Walk,
    Swim,
    Fly,
}

impl AsRef<str> for MobAnimation {
    fn as_ref(&self) -> &str {
        match self {
            MobAnimation::Walk => "walk",
            MobAnimation::Swim => "swim",
            MobAnimation::Fly => "fly",
        }
    }
}

const MOB_ANIMATIONS: &[MobAnimation] =
    &[MobAnimation::Walk, MobAnimation::Swim, MobAnimation::Fly];

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Mob {
    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, owner: &RigidBody2D) {
        let mut rng = thread_rng();
        let animated_sprite =
            unsafe { owner.get_node_as::<AnimatedSprite>("AnimatedSprite") }.unwrap();

        #[cfg(debug_assertions)]
        {
            use std::collections::HashSet;
            let frames = animated_sprite.sprite_frames().unwrap();
            let frame_names = unsafe { frames.assume_safe() }.get_animation_names();
            let known_animations = MOB_ANIMATIONS
                .iter()
                .map(|anim| anim.as_ref().to_string())
                .collect::<HashSet<_>>();
            let godot_animations = (0..frame_names.len())
                .map(|i| frame_names.get(i).to_string())
                .collect::<HashSet<_>>();

            for missing_animations in known_animations.difference(&godot_animations) {
                godot_error!(
                    "Missing animation for Mob: {}, please add it to AnimatedSprite",
                    missing_animations
                );
            }
            for unknown_animations in godot_animations.difference(&known_animations) {
                godot_error!(
                    "Unknown animation for Mob: {}, please add it to MOB_ANIMATIONS const",
                    unknown_animations
                );
            }
        }

        animated_sprite.set_animation(MOB_ANIMATIONS.choose(&mut rng).unwrap());
    }

    #[export]
    fn on_visibility_screen_exited(&self, owner: &RigidBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
