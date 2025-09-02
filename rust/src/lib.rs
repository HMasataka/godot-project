use godot::classes::Sprite2D;
use godot::classes::{INode, ISprite2D, Node, XrServer};
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct VRManager {
    base: Base<Node>,
}

#[godot_api]
impl INode for VRManager {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("VRManager初期化中...");
        self.setup_vr();
    }
}

#[godot_api]
impl VRManager {
    #[func]
    fn setup_vr(&mut self) {
        let mut xr_server = XrServer::singleton();

        // OpenXRインターフェースを探す
        let interface_count = xr_server.get_interface_count();
        godot_print!("検出されたXRインターフェース数: {}", interface_count);

        for i in 0..interface_count {
            if let Some(mut interface) = xr_server.get_interface(i) {
                let name = interface.get_name();
                godot_print!("XRインターフェース {}: {}", i, name);

                if name == StringName::from("OpenXR") {
                    godot_print!("OpenXRインターフェースが見つかりました");

                    if interface.initialize() {
                        godot_print!("OpenXRの初期化に成功しました");
                        xr_server.set_primary_interface(Some(interface));

                        // VRモードを開始
                        self.base().get_viewport().unwrap().set_use_xr(true);

                        godot_print!("VRモードが開始されました");
                    } else {
                        godot_print!("OpenXRの初期化に失敗しました");
                    }
                    break;
                }
            }
        }
    }

    #[func]
    fn get_controller_position(&self, controller_id: i32) -> Vector3 {
        godot_print!("コントローラー位置を取得: {}", controller_id);
        Vector3::ZERO
    }

    #[func]
    fn is_controller_button_pressed(&self, controller_id: i32, button_name: GString) -> bool {
        godot_print!("ボタンの状態を確認: {} - {}", controller_id, button_name);
        false
    }
}
