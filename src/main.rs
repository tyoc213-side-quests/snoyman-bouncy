extern crate pancurses;

use std::{fmt::{Display, Formatter,}};

mod parse_args;
use parse_args::{ParseError, parse_args};
// mod parse_args;

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
    fn new(frame: Frame) -> Game {
        Game {
            frame,
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
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
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
                write!(fmt, "{}", c)?;
            }
            write!(fmt, "|\n")?;
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

struct Empty(u32);

impl Iterator for Empty {
    type Item=u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(self.0)
    }
}

struct OneToTen(u32);

fn one_to_ten() -> OneToTen {
    OneToTen(1)
}

impl Iterator for OneToTen {
    type Item=u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 10 {
            None
        } else {
            let res = Some(self.0);
            self.0 += 1;
            res
        }
    }
}

struct Fibonacci(u32, u32);

fn fibo() -> Fibonacci {
    Fibonacci(1, 1)
}

impl Iterator for Fibonacci {
    type Item=u32;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.0+self.1;
        let res = Some(self.0+self.1);
        self.0 = self.1;
        self.1 = sum;
        res
    }
}

struct Doubler<I> {
    iter:I
}

impl<I> Iterator for Doubler<I>
    where
    I: Iterator<Item=u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map_or_else(|| None, |n| Some(n*2))
    }
}

fn main() {
    for i in Empty(33).take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }

    for i in one_to_ten() {
        println!("The answer to life, the universe, and everything is {}", i);
    }

    for i in fibo().take(10) {
        println!("fibo {}", i);
    }

    let orig = 1..10; // array indices start at 1
    let double_iter = Doubler {
        iter: orig,
    };
    for i in double_iter {
        println!("double {:?}", i);
    }

    println!("All done!");
}

fn main2() -> Result<(), ParseError>{
    let window = pancurses::initscr();
    let (max_x, max_y) = window.get_max_yx();
    let frame = Frame{width: max_x as u32, height: max_y as u32};
    let (width, height) = parse_args()?;
    let mut game = Game::new(Frame { width, height });
    let sleep_duration = std::time::Duration::from_millis(500);
    loop {
        window.clear();
        window.printw(game.to_string());
        window.refresh();
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
