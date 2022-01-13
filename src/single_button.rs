use wrflib::*;
use wrflib_widget::*;

#[derive(Default)]
pub struct SingleButton {
    button: Button,

    clicked: bool,
}

impl SingleButton {
    pub fn handle(&mut self, cx: &mut Cx, event: &mut Event) {
        if let ButtonEvent::Clicked = self.button.handle(cx, event) {
            self.clicked = true;
            cx.request_draw();
        }
    }

    pub fn draw(&mut self, cx: &mut Cx) {
        self.button.draw(
            cx,
            if self.clicked {
                "Hello world!"
            } else {
                "Click me!"
            },
        );
    }
}
