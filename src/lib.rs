use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Canvas {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, width: u32, height: u32) -> Result<Canvas, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
        
        canvas.set_width(width);
        canvas.set_height(height);
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
            
        Ok(Canvas {
            context,
            width,
            height,
        })
    }

    pub fn draw_circle(&self, x: f64, y: f64, radius: f64, color: &str) {
        self.context.begin_path();
        // 修正した部分：
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context
            .arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        self.context.fill();
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }
}