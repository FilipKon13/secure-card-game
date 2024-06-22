use gtk::cairo::Context;
use gtk::cairo::Matrix;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::{GdkContextExt, WidgetExt};
use gtk::Inhibit;

#[derive(Clone)]
pub struct Clickable {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    pub pixbuf: Pixbuf,
}

impl Clickable {
    pub fn new(name: String, x: f64, y: f64, angle: f64, pixbuf: Pixbuf) -> Self {
        Self {
            name,
            x,
            y,
            width: pixbuf.width() as f64,
            height: pixbuf.height() as f64,
            angle,
            pixbuf,
        }
    }

    pub fn clicked(&self, x: f64, y: f64) -> String {
        let mut mat = Matrix::identity();
        mat.translate(self.x, self.y);
        mat.rotate(self.angle.to_radians());
        mat.translate(-self.x, -self.y);
        mat.invert();

        let (dx, dy) = mat.transform_point(x, y);
        let is_clicked = dx >= self.x - self.width / 2.0
            && dx <= self.x + self.width / 2.0
            && dy >= self.y - self.height / 2.0
            && dy <= self.y + self.height / 2.0;
        if is_clicked {
            return self.name.clone();
        }
        "".to_string()
    }

    pub fn draw(&self, drawing_area: gtk::DrawingArea) {
        let x = self.x;
        let y = self.y;
        let width = self.width / 2.0;
        let height = self.height / 2.0;
        let angle = self.angle;
        let pixbuf = self.pixbuf.clone();
        drawing_area.connect_draw(move |_, cr| {
            draw_rotated_image(cr, x, y, width, height, angle, &pixbuf);

            Inhibit(false)
        });
    }
}

fn draw_rotated_image(
    cr: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    pixbuf: &Pixbuf,
) {
    cr.save();

    cr.translate(x, y);

    cr.rotate(angle.to_radians());

    // cr.translate(-x, -y);

    cr.set_source_pixbuf(pixbuf, -width, -height);
    cr.paint();

    cr.restore();
}
