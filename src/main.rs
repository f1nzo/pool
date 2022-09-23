use draw::*;

fn main() {
    let table = Table::new();
    let start = Point {
        x: 16.0,
        y: 16.0,
    };
    let target = Point {
        x: 0.0,
        y: 50.5
    };
    let success = table.hit(start, target, true);
    println!("Success: {}", success);
}

struct Table {
    // In inches
    height: f64,
    width: f64,
    goal_size: f64,
    x_landmarks: Vec<f64>,
    y_landmarks: Vec<f64>,
    inner_landmarks: Vec<Point>,
}

impl Table {
    fn new() -> Table {

        let x_landmarks: Vec<f64> = vec![4.0, 14.0, 31.0, 33.0, 41.0,];

        let y_landmarks: Vec<f64> = vec![4.0, 23.0, 42.5, 46.5, 50.5, 70.0, 90.0];

        let mut inner_landmarks: Vec<Point> = vec![];

        for x in x_landmarks.clone() {
            for y in y_landmarks.clone() {
                inner_landmarks.push(
                    Point {
                        x,
                        y,
                    }
                )
            }
        }

        return Table {
            height: 93.0,
            width: 45.0,
            goal_size: 13.0,
            x_landmarks,
            y_landmarks,
            inner_landmarks,
        };
    }

    fn hit(&self, start: Point, target: Point, draw: bool) -> bool {
        let h;
        let w;
        if target.x < start.x {
            h = target.y - start.y;
            w = start.x;
        } else if target.x > start.x {
            h = start.y - target.y;
            w = target.x - start.x;
        } else {
            println!("[ERROR] target and start x values are equal");
            return false;
        }

        if h <= 0.0 {
            println!("[ERROR] height is a negative number or 0");
            return false;
        }

        if w <= 0.0 {
            println!("[ERROR] width is a negative number or 0");
            return false;
        }

        let angle: f64 = (w/h).tanh();

        let h2 = self.height - target.y;
        let ratio = h2/h;
        let w2 = w * ratio;

        let end = Point {
            x: w2,
            y: self.height,
        };

        let goal_start = Point {
            x: self.width/2.0 - self.goal_size/2.0,
            y: self.height,
        };

        let goal_end = Point {
            x: self.width/2.0 + self.goal_size/2.0,
            y: self.height,
        };

        if draw {

            let board_height: u32 = (self.height * 10.0) as u32;
            let board_width: u32 = (self.width * 10.0) as u32;

            let canvas_board_offset = 100;

            let canvas_height: u32 = board_height + canvas_board_offset * 2;
            let canvas_width: u32 = board_width + canvas_board_offset * 2;

            let mut canvas = Canvas::new(canvas_height, canvas_width);

            // draw board
            let mut board = Drawing::new()
                .with_shape(Shape::Rectangle {
                    height: board_height,
                    width: board_width,
                })
                .with_xy(100.0, 100.0)
                .with_style(Style::filled(Color::rgb(0, 0, 0)));

            canvas.display_list.add(board);

            render::save(
                &canvas,
                "hits/svg/hit.svg",
                SvgRenderer::new(),
            )
            .expect("Failed to save");

            
            
        }

        println!("height1: {h}");
        println!("width1: {w}");
        println!("angle: {angle}");
        println!("ratio: {ratio}");
        println!("height2: {h2}");
        println!("width2: {w2}");

        if end.x > goal_start.x && end.x < goal_end.x {
            return true;
        } else {
            return false;
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}