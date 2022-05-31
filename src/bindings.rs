use std::{convert::TryInto, mem::MaybeUninit};

use bumpalo::Bump;
use mboard::{
    canvas,
    raster::{
        chunks, layer, pixels,
        position::{self, Dimensions, Scale},
    },
};

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

#[wasm_bindgen]
pub struct ImageDataService {
    bump: Bump,
}

impl ImageDataService {
    fn get_pixel_bytes<'bump>(
        pixels: bumpalo::boxed::Box<[pixels::Pixel]>,
        bump: &'bump Bump,
    ) -> bumpalo::boxed::Box<'bump, [u8]> {
        const PIXEL_SIZE_IN_U8: usize = 4;

        let pixel_bytes: &'bump mut [MaybeUninit<u8>] =
            bump.alloc_slice_fill_copy(pixels.len() * PIXEL_SIZE_IN_U8, MaybeUninit::uninit());

        for (i, pixel) in pixels.into_iter().enumerate() {
            let (r, g, b, a) = pixel.as_rgba();

            let dest_start_index = i * PIXEL_SIZE_IN_U8;
            let dest_end_index = (i + 1) * PIXEL_SIZE_IN_U8;
            MaybeUninit::write_slice(
                &mut pixel_bytes[dest_start_index..dest_end_index],
                &[r, g, b, a],
            );
        }

        let pixel_bytes =
            unsafe { std::mem::transmute::<_, bumpalo::boxed::Box<'bump, [u8]>>(pixel_bytes) };

        pixel_bytes
    }
}

#[wasm_bindgen]
impl ImageDataService {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ImageDataService {
        ImageDataService { bump: Bump::new() }
    }

    #[wasm_bindgen(js_name = "getImageDataFromCanvas")]
    pub fn get_image_data_from_canvas(
        &mut self,
        canvas: &mut Canvas,
        view: &CanvasView,
    ) -> ImageData {
        let (pixel_bytes, width) = {
            let canvas_raster = canvas.0.render_into_bump(&view.0, &self.bump);

            let width = canvas_raster.dimensions().width;
            let pixels = canvas_raster.into_pixels();

            let pixel_bytes = ImageDataService::get_pixel_bytes(pixels, &self.bump);
            (pixel_bytes, width)
        };

        let clamped_pixel_bytes = Clamped(&*pixel_bytes);

        let image_data =
            ImageData::new_with_u8_clamped_array(clamped_pixel_bytes, width.try_into().unwrap())
                .unwrap();

        std::mem::drop(clamped_pixel_bytes);
        std::mem::drop(pixel_bytes);
        self.bump.reset();

        image_data
    }

    #[wasm_bindgen(js_name = "getImageDataFromCanvasRect")]
    pub fn get_image_data_from_canvas_rect(
        &mut self,
        canvas: &mut Canvas,
        canvas_rect: &CanvasRect,
    ) -> ImageData {
        let (pixel_bytes, width) = {
            let canvas_rect_raster = canvas
                .0
                .render_canvas_rect_into_bump(canvas_rect.0, &self.bump);

            let width = canvas_rect_raster.dimensions().width;
            let pixels = canvas_rect_raster.into_pixels();

            let mut pixel_bytes =
                bumpalo::collections::Vec::<u8>::with_capacity_in(pixels.len() * 4, &self.bump);

            for pixel in pixels.into_iter() {
                let (r, g, b, a) = pixel.as_rgba();
                pixel_bytes.push(r);
                pixel_bytes.push(g);
                pixel_bytes.push(b);
                pixel_bytes.push(a);
            }
            (pixel_bytes, width)
        };

        let clamped_pixel_bytes = Clamped(pixel_bytes.as_slice());

        let image_data =
            ImageData::new_with_u8_clamped_array(clamped_pixel_bytes, width.try_into().unwrap())
                .unwrap();

        std::mem::drop(pixel_bytes);
        self.bump.reset();

        image_data
    }
}

#[wasm_bindgen]
pub struct BoxRasterChunk(chunks::BoxRasterChunk);

#[wasm_bindgen]
impl BoxRasterChunk {
    pub fn pixels(&self) -> Vec<u32> {
        self.0.pixels().iter().map(|p| p.0).collect()
    }

    pub fn width(&self) -> usize {
        self.0.dimensions().width
    }

    pub fn height(&self) -> usize {
        self.0.dimensions().height
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

impl Into<BoxRasterChunk> for chunks::BoxRasterChunk {
    fn into(self) -> BoxRasterChunk {
        BoxRasterChunk(self)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct CanvasPosition {
    pub x: i64,
    pub y: i64,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct PixelPosition {
    pub x: usize,
    pub y: usize,
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
        self.0.view_dimensions = Dimensions { width, height };
    }

    #[wasm_bindgen(js_name = "resizeCanvasSource")]
    pub fn resize_canvas_source(&mut self, width: usize, height: usize) {
        self.0.canvas_dimensions = Dimensions { width, height };
    }

    #[wasm_bindgen(js_name = "viewWidth")]
    pub fn view_width(&self) -> usize {
        self.0.view_dimensions.width
    }

    #[wasm_bindgen(js_name = "viewHeight")]
    pub fn view_height(&self) -> usize {
        self.0.view_dimensions.height
    }

    #[wasm_bindgen(js_name = "canvasWidth")]
    pub fn canvas_width(&self) -> usize {
        self.0.canvas_dimensions.width
    }

    #[wasm_bindgen(js_name = "canvasHeight")]
    pub fn canvas_height(&self) -> usize {
        self.0.canvas_dimensions.height
    }

    #[wasm_bindgen(js_name = "pinResizeCanvas")]
    pub fn pin_resize_canvas(&mut self, width: usize, height: usize) {
        self.0.pin_resize_canvas(Dimensions { width, height });
    }

    #[wasm_bindgen(js_name = "pinScaleCanvas")]
    pub fn pin_scale_canvas(&mut self, width_factor: f32, height_factor: f32) {
        self.0.pin_scale_canvas(Scale {
            width_factor,
            height_factor,
        });
    }

    #[wasm_bindgen(js_name = "anchorX")]
    pub fn anchor_x(&self) -> i64 {
        self.0.top_left.0 .0
    }

    #[wasm_bindgen(js_name = "anchorY")]
    pub fn anchor_y(&self) -> i64 {
        self.0.top_left.0 .1
    }

    #[wasm_bindgen(js_name = "transformViewToCanvas")]
    pub fn transform_view_to_canvas(&self, x: usize, y: usize) -> CanvasPosition {
        let canvas_position = self
            .0
            .transform_view_to_canvas(position::PixelPosition((x, y)));

        CanvasPosition {
            x: canvas_position.0 .0,
            y: canvas_position.0 .1,
        }
    }

    #[wasm_bindgen(js_name = "transformCanvasToView")]
    pub fn transform_canvas_to_view(&self, x: i64, y: i64) -> Option<PixelPosition> {
        let pixel_position = self
            .0
            .transform_canvas_to_view(canvas::CanvasPosition((x, y)));

        pixel_position.map(|p| PixelPosition {
            x: p.0 .0,
            y: p.0 .1,
        })
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct CanvasRect(canvas::CanvasRect);

#[wasm_bindgen]
impl CanvasRect {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64, width: usize, height: usize) -> CanvasRect {
        CanvasRect(canvas::CanvasRect {
            top_left: canvas::CanvasPosition((x, y)),
            dimensions: Dimensions { width, height },
        })
    }

    pub fn width(&self) -> usize {
        self.0.dimensions.width
    }

    pub fn height(&self) -> usize {
        self.0.dimensions.height
    }

    #[wasm_bindgen(js_name = "topLeft")]
    pub fn top_left(&self) -> CanvasPosition {
        let (x, y) = self.0.top_left.0;

        CanvasPosition { x, y }
    }

    #[wasm_bindgen(js_name = "toViewRect")]
    pub fn to_view_rect(&self, view: &CanvasView) -> Option<ViewRect> {
        self.0.to_view_rect(&view.0).map(|v| ViewRect(v))
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
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
    pub fn fill_rect(rect: &CanvasRect, pixel: &Pixel) -> RasterLayerAction {
        RasterLayerAction(layer::RasterLayerAction::fill_rect(rect.0, pixel.0))
    }

    #[wasm_bindgen(js_name = "fillOval")]
    pub fn fill_oval(rect: &CanvasRect, pixel: &Pixel) -> RasterLayerAction {
        RasterLayerAction(layer::RasterLayerAction::fill_oval(rect.0, pixel.0))
    }
}

#[wasm_bindgen]
pub struct ViewRect(canvas::ViewRect);

#[wasm_bindgen]
impl ViewRect {
    pub fn width(&self) -> usize {
        self.0.dimensions.width
    }

    pub fn height(&self) -> usize {
        self.0.dimensions.height
    }

    #[wasm_bindgen(js_name = "topLeft")]
    pub fn top_left(&self) -> PixelPosition {
        let (x, y) = self.0.top_left.0;

        PixelPosition { x, y }
    }
}

#[wasm_bindgen]
pub struct Canvas(canvas::Canvas);

const RASTER_CHUNK_SIZE: usize = 512;

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Canvas {
        Canvas(canvas::Canvas::default())
    }

    pub fn render(&mut self, view: &CanvasView) -> BoxRasterChunk {
        self.0.render(&view.0).into()
    }

    #[wasm_bindgen(js_name = "rasterizeCanvasRect")]
    pub fn rasterize_canvas_rect(&mut self, canvas_rect: CanvasRect) -> BoxRasterChunk {
        BoxRasterChunk(self.0.render_canvas_rect(canvas_rect.0))
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
            .map(|v| CanvasRect(v))
    }
}
