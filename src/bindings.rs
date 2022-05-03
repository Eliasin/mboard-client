use mboard::{
    canvas,
    raster::{chunks, layer, pixels},
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RasterChunk(chunks::RasterChunk);

#[wasm_bindgen]
impl RasterChunk {
    pub fn pixels(&self) -> Vec<u32> {
        self.0.pixels().iter().map(|p| p.0).collect()
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }

    #[wasm_bindgen(js_name = "imageData")]
    pub fn image_data(&self) -> Vec<u8> {
        self.0
            .pixels()
            .iter()
            .flat_map(|p| {
                let (r, g, b, a) = p.as_rgba();

                [r, g, b, a]
            })
            .collect()
    }
}

impl Into<RasterChunk> for chunks::RasterChunk {
    fn into(self) -> RasterChunk {
        RasterChunk(self)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct CanvasView(canvas::CanvasView);

#[wasm_bindgen]
impl CanvasView {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> CanvasView {
        CanvasView(canvas::CanvasView::new(width, height))
    }

    pub fn translate(&mut self, x: i64, y: i64) {
        self.0.translate((x, y))
    }

    #[wasm_bindgen(js_name = "resizeView")]
    pub fn resize_view(&mut self, width: usize, height: usize) {
        self.0.resize_view((width, height));
    }

    #[wasm_bindgen(js_name = "resizeCanvasSource")]
    pub fn resize_canvas_source(&mut self, width: usize, height: usize) {
        self.0.resize_canvas_source((width, height));
    }

    #[wasm_bindgen(js_name = "viewWidth")]
    pub fn view_width(&self) -> usize {
        self.0.view_dimensions().0
    }

    #[wasm_bindgen(js_name = "viewHeight")]
    pub fn view_height(&self) -> usize {
        self.0.view_dimensions().1
    }

    #[wasm_bindgen(js_name = "canvasWidth")]
    pub fn canvas_width(&self) -> usize {
        self.0.canvas_dimensions().0
    }

    #[wasm_bindgen(js_name = "canvasHeight")]
    pub fn canvas_height(&self) -> usize {
        self.0.canvas_dimensions().1
    }

    #[wasm_bindgen(js_name = "pinResizeCanvas")]
    pub fn pin_resize_canvas(&mut self, width: usize, height: usize) {
        self.0.pin_resize_canvas((width, height));
    }

    #[wasm_bindgen(js_name = "pinScaleCanvas")]
    pub fn pin_scale_canvas(&mut self, width_factor: f32, height_factor: f32) {
        self.0.pin_scale_canvas((width_factor, height_factor));
    }

    #[wasm_bindgen(js_name = "anchorX")]
    pub fn anchor_x(&self) -> i64 {
        self.0.anchor().0
    }

    #[wasm_bindgen(js_name = "anchorY")]
    pub fn anchor_y(&self) -> i64 {
        self.0.anchor().1
    }
}

#[wasm_bindgen]
pub struct CanvasRect(canvas::CanvasRect);

#[wasm_bindgen]
impl CanvasRect {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64, width: u32, height: u32) -> CanvasRect {
        CanvasRect(canvas::CanvasRect {
            top_left: (x, y),
            width,
            height,
        })
    }
}

#[wasm_bindgen]
pub struct Pixel(pixels::Pixel);

#[wasm_bindgen]
impl Pixel {
    #[wasm_bindgen(js_name = "newRgb")]
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel(pixels::Pixel::new_rgb(r, g, b))
    }

    #[wasm_bindgen(js_name = "newRgba")]
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel(pixels::Pixel::new_rgba(r, g, b, a))
    }

    #[wasm_bindgen(js_name = "newRgbNorm")]
    pub fn new_rgb_norm(r: f32, g: f32, b: f32) -> Pixel {
        Pixel(pixels::Pixel::new_rgb_norm(r, g, b))
    }

    #[wasm_bindgen(js_name = "newRgbaNorm")]
    pub fn new_rgba_norm(r: f32, g: f32, b: f32, a: f32) -> Pixel {
        Pixel(pixels::Pixel::new_rgba_norm(r, g, b, a))
    }

    #[wasm_bindgen(js_name = "asRgba")]
    pub fn as_rgba(&self) -> Vec<u8> {
        let (r, g, b, a) = self.0.as_rgba();

        vec![r, g, b, a]
    }

    #[wasm_bindgen(js_name = "asNormRgba")]
    pub fn as_norm_rgba(&self) -> Vec<f32> {
        let (r, g, b, a) = self.0.as_norm_rgba();

        vec![r, g, b, a]
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct RasterLayerAction(layer::RasterLayerAction);

#[wasm_bindgen]
impl RasterLayerAction {
    #[wasm_bindgen(js_name = "fillRect")]
    pub fn fill_rect(rect: CanvasRect, pixel: Pixel) -> RasterLayerAction {
        RasterLayerAction(layer::RasterLayerAction::fill_rect(rect.0, pixel.0))
    }

    #[wasm_bindgen(js_name = "fillOval")]
    pub fn fill_oval(rect: CanvasRect, pixel: Pixel) -> RasterLayerAction {
        RasterLayerAction(layer::RasterLayerAction::fill_oval(rect.0, pixel.0))
    }
}

#[wasm_bindgen]
pub struct Canvas(canvas::Canvas);

const RASTER_CHUNK_SIZE: usize = 512;

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Canvas {
        Canvas(canvas::Canvas::new())
    }

    pub fn render(&mut self, view: &CanvasView) -> RasterChunk {
        self.0.render(&view.0).into()
    }

    #[wasm_bindgen(js_name = "addRasterLayer")]
    pub fn add_raster_layer(&mut self) {
        self.0
            .add_layer(layer::RasterLayer::new(RASTER_CHUNK_SIZE).into());
    }

    #[wasm_bindgen(js_name = "performRasterAction")]
    pub fn perform_raster_action(
        &mut self,
        layer_num: usize,
        action: &RasterLayerAction,
    ) -> Option<CanvasRect> {
        self.0
            .perform_raster_action(layer_num, action.0)
            .map(|c| CanvasRect(c))
    }
}
