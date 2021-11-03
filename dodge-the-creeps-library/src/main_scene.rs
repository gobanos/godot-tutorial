use crate::hud::HUD;
use crate::mob::Mob;
use crate::player::Player;
use gdnative::prelude::*;
use gdnative_bindings_custom::prelude::*;
use gdnative_bindings_custom::AudioStreamPlayer;
use gdnative_bindings_custom::PathFollow2D;
use gdnative_bindings_custom::Position2D;
use gdnative_bindings_custom::RigidBody2D;
use rand::Rng;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main {
    score: u64,
    #[property]
    pub mob: Ref<PackedScene>,
}

impl Main {
    fn new(_owner: &Node) -> Self {
        Main {
            score: 0,
            mob: PackedScene::new().into_shared(),
        }
    }
}

#[methods]
impl Main {
    #[export]
    fn game_over(&self, owner: &Node) {
        godot_print!("game_over");
        unsafe {
            owner.get_node_as::<Timer>("ScoreTimer").unwrap().stop();
        }
        unsafe {
            owner.get_node_as::<Timer>("MobTimer").unwrap().stop();
        }
        unsafe {
            let hud = owner.get_node_as_instance::<HUD>("HUD").unwrap();
            hud.map(|x, o| x.show_game_over(&o)).unwrap();
        }
        unsafe {
            owner
                .get_node_as::<AudioStreamPlayer>("Music")
                .unwrap()
                .stop();
        }
        unsafe {
            owner
                .get_node_as::<AudioStreamPlayer>("DeathSound")
                .unwrap()
                .play(0.0);
        }
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        godot_print!("new_game");
        self.score = 0;
        let start_position = unsafe { owner.get_node_as::<Position2D>("StartPosition").unwrap() };
        unsafe {
            let player = owner.get_node_as_instance::<Player>("Player").unwrap();
            player
                .map_mut(|x, o| x.start(&o, start_position.position()))
                .unwrap();
        }
        unsafe {
            let hud = owner.get_node_as_instance::<HUD>("HUD").unwrap();
            hud.map(|x, o| x.update_score(&o, self.score)).unwrap();
            hud.map(|x, o| x.show_message(&o, "Get Ready".into()))
                .unwrap();
        }
        unsafe { owner.get_tree().unwrap().assume_unique() }.call_group("mobs", "queue_free", &[]);
        unsafe {
            owner
                .get_node_as::<AudioStreamPlayer>("Music")
                .unwrap()
                .play(0.0);
        }
        unsafe {
            owner.get_node_as::<Timer>("StartTimer").unwrap().start(0.0);
        }
        godot_print!("new_game::completed");
    }

    #[export]
    fn on_start_timer_timeout(&self, owner: &Node) {
        godot_print!("on_start_timer_timeout");
        unsafe {
            owner.get_node_as::<Timer>("ScoreTimer").unwrap().start(0.0);
        }
        unsafe {
            owner.get_node_as::<Timer>("MobTimer").unwrap().start(0.0);
        }
    }

    #[export]
    fn on_score_timer_timeout(&mut self, owner: &Node) {
        godot_print!("on_score_timer_timeout");
        self.score += 1;
        unsafe {
            let hud = owner.get_node_as_instance::<HUD>("HUD").unwrap();
            hud.map(|x, o| x.update_score(&o, self.score)).unwrap();
        }
    }

    #[export]
    fn on_mob_timer_timeout(&self, owner: &Node) {
        godot_print!("on_mob_timer_timeout");
        let mut rng = rand::thread_rng();
        let mob_spawn_location = unsafe {
            owner
                .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation")
                .unwrap()
        };
        mob_spawn_location.set_offset(rng.gen_range(0.0..(u32::MAX as f64)));
        let mob_scene = unsafe {
            self.mob
                .assume_safe()
                .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                .unwrap()
                .assume_unique()
                .try_cast::<RigidBody2D>()
                .unwrap()
                .into_shared()
                .assume_safe()
        };
        owner.add_child(mob_scene, false);

        let direction =
            mob_spawn_location.rotation() + PI / 2.0 + rng.gen_range((-PI / 4.0)..(PI / 4.0));

        mob_scene.set_position(mob_spawn_location.position());
        mob_scene.set_rotation(direction);

        let mob = mob_scene.cast_instance::<Mob>().unwrap();
        mob.map(|x, o| {
            let linear_velocity = Vector2::new(rng.gen_range(x.min_speed..x.max_speed), 0.0)
                .rotated(Angle::radians(direction as f32));
            o.set_linear_velocity(linear_velocity);
        })
        .unwrap();
    }
}
