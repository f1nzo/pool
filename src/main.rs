use turtle::Turtle;

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

        println!("height1: {h}");
        println!("width1: {w}");
        println!("angle: {angle}");
        println!("ratio: {ratio}");
        println!("height2: {h2}");
        println!("width2: {w2}");

        if draw {
            let mut turtle = Turtle::new();
            // use turtle to draw the table
            turtle.set_speed("instant");
            turtle.set_pen_size(2.0);
            turtle.set_pen_color("black");
            turtle.set_fill_color("black");
            turtle.pen_up();
            turtle.go_to([start.x, start.y]);
            turtle.pen_down();
            turtle.go_to([target.x, target.y]);
            turtle.pen_up();
            turtle.go_to([end.x, end.y]);
            turtle.pen_down();
            turtle.go_to([goal_start.x, goal_start.y]);
            turtle.go_to([goal_end.x, goal_end.y]);
            turtle.go_to([end.x, end.y]);
            turtle.pen_up();
            turtle.set_pen_size(1.0);
            turtle.set_pen_color("red");
            turtle.set_fill_color("red");
            turtle.set_pen_size(2.0);
            turtle.set_pen_color("black");
            turtle.set_fill_color("black");
            turtle.go_to([0.0, 0.0]);
            turtle.pen_down();
        }

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