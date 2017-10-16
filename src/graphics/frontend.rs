use geom::{Circle, Rectangle, Transform};
use graphics::{Bridge, Camera, Color, Drawable, TextureRegion, WHITE};
use std::sync::{Arc, Mutex};

pub struct Frontend {
    bridge: Arc<Mutex<Bridge>>,
    cam: Camera,
    ui_mode: bool
}

impl Frontend {
    pub fn new(bridge: Arc<Mutex<Bridge>>, cam: Camera) -> Frontend {
        Frontend {
            bridge: bridge,
            cam: cam,
            ui_mode: false
        }
    }

    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }

    pub fn get_ui_mode(&self) -> bool {
        self.ui_mode
    }

    pub fn set_ui_mode(&mut self, ui_mode: bool) {
        self.ui_mode = ui_mode;
    }

    fn camera(&self) -> Transform {
        self.cam.opengl
    }

    pub fn clear(&self, color: Color) {
        self.bridge.lock().unwrap().add((Drawable::Clear, Transform::identity(), color));
    }

    pub fn present(&self) {
        self.bridge.lock().unwrap().add((Drawable::Present, Transform::identity(), WHITE));
    }

    pub fn draw_image(&self, image: TextureRegion, area: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans 
            * Transform::translate(area.top_left()) 
            * Transform::scale(area.size());
        let call = (Drawable::Image((image.get_id(), image.source_size(), image.get_region())), trans, col);
        self.bridge.lock().unwrap().add(call);
    }

    pub fn draw_rect(&self, rect: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        let call = (Drawable::Rect(rect), trans, col);
        self.bridge.lock().unwrap().add(call);
    }

    pub fn draw_circle(&self, circ: Circle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        let call = (Drawable::Circ(circ), trans, col);
        self.bridge.lock().unwrap().add(call);
    }
}
