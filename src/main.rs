use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum VertDir {
    Up,
    Down,
}
#[derive(Debug)]
enum  HorizDir {
    Left,
    Right
}

#[derive(Debug)]
struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    hor_dir: HorizDir,
}

#[derive(Debug)]

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 12,
                height: 7,
            },
            ball: Ball {
                x: 4,
                y: 4,
                vert_dir: VertDir::Up,
                hor_dir: HorizDir::Left,
            },
        }
    }

    fn mv(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }

    fn step(&mut self) {
        self.mv();
    }
}

impl Display for Game {
    fn fmt(&self, mut fmt: &mut Formatter) -> std::fmt::Result {
        // write!(fmt, "+");
        // for _ in 0..self.frame.width {
        //     write!(fmt, "-");
        // }
        // write!(fmt, "+\n")
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+");
            for _ in 0..self.frame.width {
                write!(fmt, "-");
            }
            write!(fmt, "+\n")
        };
        
        // top_bottom();
        let _r1 = top_bottom(&mut fmt);
        for row in 0..self.frame.height {
            let _r2 = write!(fmt, "|");
            for column in 0..self.frame.width {
                let c = if row == self.ball.y && column == self.ball.x {
                    'o' 
                } else {
                    ' '
                };
                write!(fmt, "{}", c);
            }
            let _r3 = write!(fmt, "|\n");
        }
        let _r4 = top_bottom(&mut fmt);
        _r4
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.hor_dir = HorizDir::Right;
        } else if self.x >= frame.width - 1 {
            self.hor_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down
        } else if self.y == frame.height -1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.hor_dir {
            HorizDir::Right => self.x += 1,
            HorizDir::Left => self.x -= 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}


fn main() {
    let mut g = Game::new();
    let sleep_duration = std::time::Duration::from_millis(630);
    loop {
        println!("{}", g);
        g.step();
        std::thread::sleep(sleep_duration); 
    }
    
}
