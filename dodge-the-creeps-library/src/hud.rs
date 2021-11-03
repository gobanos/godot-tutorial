use gdnative::prelude::*;
use gdnative_bindings_custom::prelude::*;
use gdnative_bindings_custom::CanvasLayer;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_hud)]
pub struct HUD;

impl HUD {
    fn new(_owner: &CanvasLayer) -> Self {
        HUD
    }

    fn register_hud(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "start_game",
            args: &[],
        });
    }
}

#[methods]
impl HUD {
    #[export]
    pub fn show_message(&self, owner: &CanvasLayer, text: GodotString) {
        let message_label = unsafe { owner.get_node_as::<Label>("Message") }.unwrap();
        message_label.set_text(text);
        message_label.show();
        let message_timer = unsafe { owner.get_node_as::<Timer>("MessageTimer") }.unwrap();
        message_timer.start(0.0);
    }

    #[export]
    pub fn show_game_over(&self, owner: &CanvasLayer) {
        self.show_message(owner, "Game over".into());
        let message_timer = unsafe { owner.get_node_as::<Timer>("MessageTimer") }.unwrap();
        message_timer
            .connect(
                "timeout",
                unsafe { owner.assume_unique() },
                "show_game_over_1",
                VariantArray::new_shared(),
                Object::CONNECT_ONESHOT | Object::CONNECT_DEFERRED,
            )
            .unwrap();
    }

    #[export]
    fn show_game_over_1(&self, owner: &CanvasLayer) {
        let message_label = unsafe { owner.get_node_as::<Label>("Message") }.unwrap();
        message_label.set_text("Dodge the Creeps!");
        message_label.show();
        let one_shot_timer = unsafe { owner.get_tree().unwrap().assume_unique() }
            .create_timer(1.0, true)
            .unwrap();

        unsafe { one_shot_timer.assume_safe() }
            .connect(
                "timeout",
                unsafe { owner.assume_unique() },
                "show_game_over_2",
                VariantArray::new_shared(),
                Object::CONNECT_ONESHOT,
            )
            .unwrap();
    }

    #[export]
    fn show_game_over_2(&self, owner: &CanvasLayer) {
        let start_button = unsafe { owner.get_node_as::<Button>("StartButton") }.unwrap();
        start_button.show();
    }

    #[export]
    pub fn update_score(&self, owner: &CanvasLayer, score: u64) {
        let score_label = unsafe { owner.get_node_as::<Label>("ScoreLabel") }.unwrap();
        score_label.set_text(score.to_string());
    }

    #[export]
    fn on_start_button_pressed(&self, owner: &CanvasLayer) {
        let start_button = unsafe { owner.get_node_as::<Button>("StartButton") }.unwrap();
        start_button.hide();
        owner.emit_signal("start_game", &[]);
    }

    #[export]
    fn on_message_timer_timeout(&self, owner: &CanvasLayer) {
        let message_label = unsafe { owner.get_node_as::<Label>("Message") }.unwrap();
        message_label.hide();
    }
}
