use rhai::{Engine, Scope, Dynamic, Map};
use crate::led_driver::{LedController, Color};

pub struct ScriptedEffect {
    engine: Engine,
    script_path: String,
}

impl ScriptedEffect {
    pub fn new(path: &str) -> Self {
        Self {
            engine: Engine::new(),
            script_path: path.to_string(),
        }
    }

    pub fn run(&self, controller: &mut LedController, tick: f32) {
        let mut scope = Scope::new();
        
        // Pass variables into the user's script
        let result: Dynamic = self.engine.call_fn(
            &mut scope, 
            &self.engine.compile_file(self.script_path.clone().into()).unwrap(),
            "update", 
            (tick as f64, 24_i64)
        ).unwrap();

        // Convert script output back to LED colors
        let frame: Vec<Dynamic> = result.cast::<Vec<Dynamic>>();
        for (i, val) in frame.into_iter().enumerate() {
            let map = val.cast::<Map>();
            let r = map["r"].clone().cast::<i64>() as u8;
            let g = map["g"].clone().cast::<i64>() as u8;
            let b = map["b"].clone().cast::<i64>() as u8;
            
            controller.set_zone(i, Color::new(r, g, b));
        }
    }
}