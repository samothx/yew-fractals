use js_sys::Object;
use seed::prelude::web_sys::{HtmlCanvasElement, ImageData};
use seed::prelude::JsValue;
use seed::{canvas, prelude::*, window};
use seed::{log, spawn_local};
use wasm_bindgen_futures::JsFuture;
use web_sys::{PermissionStatus, PermissionState};

use super::{fractal::Points, FractalType, Model, BACKGROUND_COLOR};
use serde::{Deserialize, Serialize};

const COLOR_MAX: u32 = 0x00FF_FFFF;
const COLOR_MIN: u32 = 0x00FF_FFFF;

const START_HUE: u32 = 0;
const DEFAULT_SATURATION: f32 = 1.0;
const DEFAULT_LIGHTNESS: f32 = 0.5;

const HUE_OFFSET: f32 = 0.0;
const HUE_RANGE: f32 = 300.0;

#[derive(Serialize, Deserialize)]
struct QueryObject {
    pub name: String,
}

pub struct Canvas {
    canvas: HtmlCanvasElement,
    steps: u32,
    width: u32,
}

impl Canvas {
    pub fn new(model: &Model) -> Self {
        Self {
            canvas: canvas("canvas").expect("Canvas not found"),
            steps: match model.config.active_config {
                FractalType::JuliaSet => model.config.julia_set_cfg.max_iterations,
                FractalType::Mandelbrot => model.config.mandelbrot_cfg.max_iterations,
            },
            width: model.width,
        }
    }

    pub fn copy_to_clipboard(&self) {
        // TODO: understand Promises & Closures in web-sys
        log!("copy_to_clipboard: entered");
        let query_obj = Object::from(
            JsValue::from_serde(&QueryObject {
                name: String::from("clipboard-write"),
            })
            .expect("unable to create JSValue"),
        );
        log!(format!("copy_to_clipboard: got query_object: {:?}", query_obj));
        match window()
            .navigator()
            .permissions()
            .expect("no permissipons found in navigator")
            .query(&query_obj)
        {
            Ok(result) => {
                log!("copy_to_clipboard: query permission returned ok");
                spawn_local(async move {
                    let resolve = Closure::wrap(Box::new(|js_value: JsValue|  {
                        log!(format!("copy_to_clipboard: got Result {:?}", js_value));
                        let status = PermissionStatus::from(js_value);
                        log!(format!("copy_to_clipboard: got PermissionStatus {:?}", status.state()));
                        if (status.state() == PermissionState::Granted) {
                            // TODO: do something 
                        }
                    }) as Box<dyn FnMut(JsValue)>);

                    let reject = Closure::wrap(Box::new(|js_value: JsValue| {
                        log!(format!(
                            "copy_to_clipboard: get permission: error {}",
                            js_value.as_string().unwrap_or("None".to_string())
                        ));
                    }) as Box<dyn FnMut(JsValue)>);

                    let promise = result.then2(&resolve, &reject);
                    let res = JsFuture::from(promise).await;
                    log!(format!("copy_to_clipboard: spawned future returned {:?}", res));
                });
                log!("copy_to_clipboard: spawned future locally");
            },
            Err(err) => {
                log!(format!(
                    "copy_to_clipboard: error from query permissions, msg: {}",
                    err.as_string().unwrap_or("None".to_string())
                ));    
            }
        }

    }

    pub fn clear_canvas(&self, model: &Model) {
        log!("Clear Canvas");

        if model.height != self.canvas.height() {
            self.canvas.set_height(model.height);
        }
        if model.width != self.canvas.width() {
            self.canvas.set_width(model.width);
        }

        let ctx = seed::canvas_context_2d(&self.canvas);
        // ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(model.background_color.as_str()));
        ctx.fill_rect(0.into(), 0.into(), model.width.into(), model.height.into());

        // ctx.fill();
        // ctx.stroke();
    }

    pub fn draw_results(&self, points: &Points) {
        let mut x = points.x_start;
        let mut y = points.y_start;
        let ctx = seed::canvas_context_2d(&self.canvas);
        ctx.set_fill_style(&JsValue::from_str("FFFFFF"));

        let mut last_color = "".to_string();
        points.values[0..points.num_points]
            .iter()
            .for_each(|value| {
                let color = if *value >= self.steps - 1 {
                    BACKGROUND_COLOR.to_string()
                } else {
                    self.iterations_as_hue_to_rgb(*value)
                };
                if color != last_color {
                    // log!(format!("draw_result: color: {} pos: {},{}", color, x, y));
                    ctx.set_fill_style(&JsValue::from_str(color.as_str()));
                    last_color = color;
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

        let ctx = seed::canvas_context_2d(&self.canvas);

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
        let ctx = seed::canvas_context_2d(&self.canvas);
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

    #[allow(clippy::cast_precision_loss)]
    fn iterations_as_hue_to_rgb(&self, iterations: u32) -> String {
        Self::hue_to_rgb(
            (iterations as f32).mul_add(HUE_RANGE / self.steps as f32, HUE_OFFSET) % 360.0,
        )
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn hue_to_rgb(hue: f32) -> String {
        const TMP: f32 = 2.0 * DEFAULT_LIGHTNESS - 1.0;
        const C: f32 = (1.0 - if TMP >= 0.0 { TMP } else { -TMP }) * DEFAULT_SATURATION;
        const M: f32 = DEFAULT_LIGHTNESS - C / 2.0;
        let x = C * (1.0 - f32::abs((hue / 60.0) % 2.0 - 1.0));

        let (r, g, b) = match hue as u32 {
            0..=59 => (C, x, 0.0),
            60..=119 => (x, C, 0.0),
            120..=179 => (0.0, C, x),
            180..=239 => (0.0, x, C),
            240..=299 => (x, 0.0, C),
            _ => (C, 0.0, x),
        };

        let (r, g, b) = (
            f32::floor((r + M) * 255.0).abs() as u32,
            f32::floor((g + M) * 255.0).abs() as u32,
            f32::floor((b + M) * 255.0).abs() as u32,
        );

        format!("#{:0>2X}{:0>2X}{:0>2X}", r % 0x100, g % 0x100, b % 0x100)
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> String {
        // see: https://www.rapidtables.com/convert/color/hsl-to-rgb.html

        assert!((0.0..=1.0).contains(&saturation));
        assert!((0.0..=1.0).contains(&lightness));

        let safe_hue = if hue >= 360.0 { hue % 360.0 } else { hue };

        let c = (1.0 - f32::abs(2.0 * lightness - 1.0)) * saturation;
        let x = c * (1.0 - ((safe_hue / 60.0) % 2.0 - 1.0).abs());
        let m = lightness - c / 2.0;
        let (r, g, b) = match hue as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => {
                panic!("invalid hue value");
            }
        };

        let (r, g, b) = (
            f32::floor((r + m) * 255.0).abs() as u32,
            f32::floor((g + m) * 255.0).abs() as u32,
            f32::floor((b + m) * 255.0).abs() as u32,
        );

        format!("#{:X}{:X}{:X}", r % 0x100, g % 0x100, b % 0x100)
    }
}

#[cfg(test)]
mod test {
    use super::Canvas;
    #[test]
    fn test_iterations_as_hue_to_rgb() {
        assert_eq!(Canvas::hue_to_rgb(0.0), "#FF0000");
        assert_eq!(Canvas::hue_to_rgb(60.0), "#FFFF00");
        assert_eq!(Canvas::hue_to_rgb(120.0), "#00FF00");
        assert_eq!(Canvas::hue_to_rgb(180.0), "#00FFFF");
        assert_eq!(Canvas::hue_to_rgb(240.0), "#0000FF");
        assert_eq!(Canvas::hue_to_rgb(300.0), "#FF00FF");
        assert_eq!(Canvas::hue_to_rgb(360.0), "#FF0000");
        assert_eq!(Canvas::hue_to_rgb(340.0), "#FF0055");
        // TODO: Tests for hsl_to_rgb
    }
}
