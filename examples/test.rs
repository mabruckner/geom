#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate geom;

use conrod::{
    color,
    Colorable,
    CommonBuilder,
    UpdateArgs,
    WidgetKind,
    Backend,
    NodeIndex,
    Circle,
 Labelable, Positionable, Sizeable, Theme, Widget,   Line
};

use geom::{
    Point,
    Shape
};

fn approximate(shape: &Shape, num: i32) -> Vec<Point>
{
    (0..(num+1)).map(|val|{
        shape.param((val as f64) / (num as f64))
    }).collect::<Vec<Point>>()
}

fn approxpts(shapes: &Vec<Shape>, numper: i32) -> Vec<Point>
{
    shapes.iter().flat_map(|shape|{
        approximate(shape,numper)
    }).collect()
}

fn closepts(shapes: &Vec<Shape>, point: Point) -> Vec<Point>
{
    shapes.iter().flat_map(|shape|{
        shape.nearpoints(point).iter().map(|&(_,t)|{
            shape.param(t)
        }).collect::<Vec<Point>>()
    }).collect()
}

#[derive(PartialEq,Debug)]
struct State
{
    rect_index: Option<NodeIndex>,
    indices: Vec<NodeIndex>
}

struct Renderer
{
    common:CommonBuilder,
}

pub const KIND: WidgetKind = "Renderer";

impl  Renderer
{
    fn new() -> Renderer
    {
        Renderer{
            common:CommonBuilder::new(),
        }
    }
}

impl Widget for Renderer
{
    type State = State;
    type Style = ();
    fn common(&self) -> &CommonBuilder
    {
        &self.common
    }
    fn common_mut(&mut self) -> &mut CommonBuilder
    {
        &mut self.common
    }
    fn unique_kind(&self) -> &'static str
    {
        KIND
    }
    fn init_state(&self) -> Self::State
    {
        State {
            rect_index: None,
            indices:Vec::new()
        }
    }
    fn style(&self) -> Self::Style
    {
        ()
    }
    fn update<B: Backend>(self, args: UpdateArgs<Self,B>)
    {
        let UpdateArgs{ idx, state, rect, style, mut ui, ..} = args;
        //println!("{:?}",rect);
        let mut shapes = vec![
            Shape::Segment(Point{x:0.0,y:0.0},Point{x:100.0,y:50.0}),
            Shape::Arc{center:Point{x:0.0,y:0.0},radius:100.0,start:0.0,circ:1.0}];
        let points = approxpts(&shapes,10);
        let mut current = 0;
        let mut indices = state.view().indices.clone();
        let prevlen = indices.len();
/*        for shape in shapes {
            match shape {
                Shape::Segment(a,b) => {
                    let index;
                    if current < indices.len() {
                        index = indices[current];
                    } else {
                        index = ui.new_unique_node_index();
                        indices.push(index);
                    }
                    Line::abs([a.x,a.y],[b.x,b.y])
                        .color(color::WHITE)
                        .middle_of(idx)
                        .set(index, &mut ui);
                    current = current + 1;
                }
                _ => ()
            }
        }*/
        for pt in points {
            let index;
            if current < indices.len() {
                index = indices[current];
            } else {
                index = ui.new_unique_node_index();
                indices.push(index);
            }
            Circle::fill(2.0)
                .color(color::WHITE)
                .middle_of(idx)
                .x_y(pt.x,pt.y)
                .set(index, &mut ui);
            current = current + 1;
        }
        if indices.len() > prevlen {
            state.update(|state| state.indices = indices)
        }
    }
}

fn main() {
    use conrod::{Labelable, Positionable, Sizeable, Theme, Widget};
    use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

    // Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
    type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
    type Ui = conrod::Ui<Backend>;

    // Construct the window.
    let window: PistonWindow = WindowSettings::new("Geom Test App 1", [200, 200])
        .exit_on_esc(true).build().unwrap();

    // construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };
    let mut count = 0;
    let mut shapes = vec![
        Shape::Segment(Point{x:0.0,y:0.0},Point{x:10.0,y:5.0}),
        Shape::Arc{center:Point{x:0.0,y:0.0},radius:10.0,start:0.0,circ:1.0}];
    // Poll events from the window.
    for event in window.ups(60) {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|ref mut ui| {

            // Generate the ID for the Button COUNTER.
            widget_ids!(CANVAS, RENDERER);

            // Create a background canvas upon which we'll place the button.
            conrod::Canvas::new().pad(40.0).set(CANVAS, ui);

            // Draw the button and increment `count` if pressed.
/*            conrod::Button::new()
                .middle_of(CANVAS)
                .w_h(80.0, 80.0)
                .label(&count.to_string())
                .react(|| count += 1)
                .set(COUNTER, ui);*/

            Renderer::new()
                .set(RENDERER, ui);
        }));
        event.draw_2d(|c, g| ui.draw_if_changed(c, g));
    }
}

