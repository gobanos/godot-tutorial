use gdnative::core_types::Size2;
use gdnative::prelude::*;
use gdnative_bindings_custom::prelude::*;
use gdnative_bindings_custom::InputEventScreenTouch;
use gdnative_bindings_custom::{AnimatedSprite, Area2D, CollisionShape2D, PhysicsBody2D};

#[derive(NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 400.0)]
    pub speed: f32,
    screen_size: Size2,
    target: Vector2,
}

impl Player {
    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Player {
            speed: 400.0,
            screen_size: Size2::default(), // Set in `_ready`
            target: Vector2::default(),    // Set in `start`
        }
    }

    fn register_player(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hit",
            args: &[],
        });
    }
}

#[derive(Debug, Copy, Clone)]
enum PlayerAnimation {
    Walk,
    Up,
}

impl AsRef<str> for PlayerAnimation {
    fn as_ref(&self) -> &str {
        match self {
            PlayerAnimation::Walk => "walk",
            PlayerAnimation::Up => "up",
        }
    }
}

#[methods]
impl Player {
    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        self.screen_size = owner.get_viewport_rect().size;
        owner.hide();

        #[cfg(debug_assertions)]
        {
            use std::collections::HashSet;
            const PLAYER_ANIMATIONS: &[PlayerAnimation] =
                &[PlayerAnimation::Walk, PlayerAnimation::Up];

            let animated_sprite =
                unsafe { owner.get_node_as::<AnimatedSprite>("AnimatedSprite") }.unwrap();
            let frames = animated_sprite.sprite_frames().unwrap();
            let frame_names = unsafe { frames.assume_safe() }.get_animation_names();
            let known_animations = PLAYER_ANIMATIONS
                .iter()
                .map(|anim| anim.as_ref().to_string())
                .collect::<HashSet<_>>();
            let godot_animations = (0..frame_names.len())
                .map(|i| frame_names.get(i).to_string())
                .collect::<HashSet<_>>();

            for missing_animations in known_animations.difference(&godot_animations) {
                godot_error!(
                    "Missing animation for Player: {}, please add it to AnimatedSprite",
                    missing_animations
                );
            }
            for unknown_animations in godot_animations.difference(&known_animations) {
                godot_error!(
                    "Unknown animation for Player: {}, please add it to PLAYER_ANIMATIONS const",
                    unknown_animations
                );
            }
        }
    }

    #[export]
    fn _input(&mut self, _owner: &Area2D, event: Variant) {
        if let Some(event) = event.try_to_object::<InputEventScreenTouch>() {
            let event = unsafe { event.assume_safe() };
            if event.is_pressed() {
                self.target = event.position();
            }
        }
    }

    #[export]
    fn _process(&self, owner: &Area2D, delta: f32) {
        let animated_sprite =
            unsafe { owner.get_node_as::<AnimatedSprite>("AnimatedSprite") }.unwrap();

        let mut velocity = if owner.position().distance_to(self.target) > 10.0 {
            self.target - owner.position()
        } else {
            Vector2::default()
        };

        if velocity.length() > 0.0 {
            velocity = velocity.normalize() * self.speed;
            animated_sprite.play("", false);
        } else {
            animated_sprite.stop();
        }

        owner.set_position((owner.position() + velocity * delta).clamp(
            Vector2::new(0.0, 0.0),
            Vector2::new(self.screen_size.width, self.screen_size.height),
        ));

        if velocity.x != 0.0 {
            animated_sprite.set_animation(PlayerAnimation::Walk);
            animated_sprite.set_flip_v(false);
            animated_sprite.set_flip_h(velocity.x < 0.0);
        } else if velocity.y != 0.0 {
            animated_sprite.set_animation(PlayerAnimation::Up);
            animated_sprite.set_flip_v(velocity.y > 0.0);
        }
    }

    #[export]
    pub fn start(&mut self, owner: &Area2D, position: Vector2) {
        owner.set_position(position);
        self.target = position;
        owner.show();

        let collision_shape =
            unsafe { owner.get_node_as::<CollisionShape2D>("CollisionShape2D") }.unwrap();
        collision_shape.set_disabled(false);
    }

    #[export]
    fn on_player_body_entered(&self, owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);
        let collision_shape =
            unsafe { owner.get_node_as::<CollisionShape2D>("CollisionShape2D") }.unwrap();
        collision_shape.set_deferred("disabled", true);
    }
}
