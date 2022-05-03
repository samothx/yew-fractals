use super::fractal::Points;
use crate::components::root::Config;
use crate::work::colors::{ColorRange, BACKGROUND_COLOR};
use crate::work::fractal::FractalType;
use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    iterations: u32,
    width: u32,
    color_range: ColorRange,
}

impl Canvas {
    pub fn new(canvas: HtmlCanvasElement, config: &Config, width: u32) -> Self {
        let (iterations, color_cfg_name) = match config.active_config {
            FractalType::JuliaSet => (
                config.julia_set_cfg.max_iterations,
                config.julia_set_cfg.color_cfg_name.as_ref(),
            ),
            FractalType::Mandelbrot => (
                config.mandelbrot_cfg.max_iterations,
                config.mandelbrot_cfg.color_cfg_name.as_ref(),
            ),
        };

        let color_range = if let Some(color_cfg_name) = color_cfg_name {
            if let Some(color_range) = config.color_cfg.get(color_cfg_name.as_str()) {
                color_range.clone()
            } else {
                ColorRange::default()
            }
        } else {
            ColorRange::default()
        };

        Self {
            canvas,
            iterations,
            width,
            color_range,
        }
    }

    pub fn clear_canvas(&mut self, width: u32, height: u32) {
        info!("Clear Canvas");
        if height != self.canvas.height() {
            self.canvas.set_height(height);
        }
        if width != self.canvas.width() {
            self.canvas.set_width(width);
        }
        self.width = width;

        let ctx = self.get_2d_context();
        // ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
        ctx.fill_rect(0.into(), 0.into(), width.into(), height.into());
    }

    pub fn draw_results(&self, points: &Points) {
        let mut x = points.x_start;
        let mut y = points.y_start;

        let ctx = self.get_2d_context();
        ctx.set_fill_style(&JsValue::from_str("FFFFFF"));

        let mut last_value = self.iterations + 2;
        points.values[0..points.num_points]
            .iter()
            .for_each(|value| {
                if *value != last_value {
                    last_value = *value;
                    let color = if *value > self.iterations {
                        BACKGROUND_COLOR.to_string()
                    } else {
                        self.iterations_as_hue_to_rgb(*value)
                    };
                    // log!(format!("draw_result: color: {} pos: {},{}", color, x, y));
                    ctx.set_fill_style(&JsValue::from_str(color.as_str()));
                }
                ctx.fill_rect(x.into(), y.into(), 1.0, 1.0);

                x += 1;
                if x >= self.width {
                    x = 0;
                    y += 1;
                }
            });
    }

    pub fn draw_frame(&self, x_start: u32, y_start: u32, x_end: u32, y_end: u32) -> ImageData {
        // log!(format!("draw_frame: ({},{}),({},{})", x_start,y_start, x_end, y_end));

        let ctx = self.get_2d_context();

        // TODO: try this again later
        /*
        let image_width = f64::max(canvas_right - canvas_left, 1.0);
        let image_height = f64::max(canvas_bottom - canvas_top, 1.0);
        log!(format!("draw_frame: image coords: ({},{}),({},{})", canvas_left,canvas_top, image_width, image_height));
        let image_data =
            ctx.get_image_data(canvas_left, canvas_top, image_width, image_height)
                .expect("failed to retrieve image data")
                .dyn_into::<ImageData>().expect("Failed to cast to ImageData");
        */
        let image_data = ctx
            .get_image_data(
                0.0,
                0.0,
                self.canvas.width().into(),
                self.canvas.height().into(),
            )
            .expect("failed to retrieve image data")
            .dyn_into::<ImageData>()
            .expect("Failed to cast to ImageData");

        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#FFFFFF"));
        ctx.move_to(x_start.into(), y_start.into());
        ctx.line_to(x_end.into(), y_start.into());
        ctx.line_to(x_end.into(), y_end.into());
        ctx.line_to(x_start.into(), y_end.into());
        ctx.line_to(x_start.into(), y_start.into());
        ctx.stroke();
        image_data
    }

    pub fn undraw(&self, image_data: &ImageData) {
        // log!(format!("undraw: ({},{}) width: {} height: {}", x_start,y_start, image_data.width(), image_data.height()));
        let ctx = self.get_2d_context();
        ctx.put_image_data(image_data, 0.0, 0.0)
            .expect("cannot draw image data");
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn viewport_to_canvas_coords(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let bounding_rect = self.canvas.get_bounding_client_rect();
        let scale_x = f64::from(self.canvas.width()) / bounding_rect.width();
        let scale_y = f64::from(self.canvas.height()) / bounding_rect.height();
        let canvas_x = (f64::from(x) - bounding_rect.left()) * scale_x;
        let canvas_y = (f64::from(y) - bounding_rect.top()) * scale_y;
        if canvas_x >= 0.0
            && canvas_x < f64::from(self.canvas.width())
            && canvas_y >= 0.0
            && canvas_y < f64::from(self.canvas.height())
        {
            Some((canvas_x.abs() as u32, canvas_y.abs() as u32))
        } else {
            None
        }
    }

    #[inline]
    fn get_2d_context(&self) -> CanvasRenderingContext2d {
        let tmp1: Object = self
            .canvas
            .get_context("2d")
            .map_or_else(
                |err| panic!("failed to retrieve CanvasRenderingContext2d {:?}", err),
                |v| v,
            )
            .expect("2d context not found in canvas");
        let tmp2: &JsValue = tmp1.as_ref();
        tmp2.clone()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Failed to cast to CanvasRenderingContext2d")
    }

    #[allow(clippy::cast_precision_loss)]
    fn iterations_as_hue_to_rgb(&self, iterations: u32) -> String {
        let color_rgb = match &self.color_range {
            ColorRange::Hsl(range) => range
                .percent_of((iterations as f32 / self.iterations as f32).min(1.0))
                .to_rgb(),
            ColorRange::Rgb(range) => {
                range.percent_of((iterations as f32 / self.iterations as f32).min(1.0))
            }
        };
        color_rgb.to_string()
    }
}
