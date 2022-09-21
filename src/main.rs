fn main() {
    let table = Table::new();
    let start = Point {
        x: 4.0,
        y: 4.0,
    };
    let target = Point {
        x: 0.0,
        y: 50.5
    };
    let sucess = table.hit(start, target);
}

struct Table {
    // In inches
    height: f32,
    width: f32,
    goal_size: f32,
    x_landmarks: Vec<f32>,
    y_landmarks: Vec<f32>,
    inner_landmarks: Vec<Point>,
}

impl Table {
    fn new() -> Table {

        let x_landmarks = vec![4.0, 14.0, 31.0, 33.0, 41.0,];

        let y_landmarks = vec![4.0, 23.0, 42.5, 46.5, 50.5, 70.0, 90.0,];

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

    fn hit(&self, start: Point, target: Point) -> bool {
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

        let angle = (w/h).tanh();

        println!("height: {h}");
        println!("width: {w}");
        println!("angle: {angle}");

        true
    }
}

struct Point {
    x: f32,
    y: f32,
}