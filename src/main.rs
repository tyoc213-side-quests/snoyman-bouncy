extern crate pancurses;

use std::{fmt::{Display, Formatter,}, option::IntoIter};

mod parse_args;
use parse_args::{ParseError, parse_args};
// mod parse_args;



fn call_fn<F>(f: F) where F: Fn() {
    f()
}

fn call_fn_mut<F>(mut f: F) where F: FnMut() {
    f()
}

fn call_fn_once<F>(f: F) where F: FnOnce() {
    f()
}

fn main() {
    let name = String::from("Alice");
    let say_hi = || println!("Hello, {}", name);
    call_fn(say_hi);
    call_fn_mut(say_hi);
    call_fn_once(say_hi);
    main_move();
}

fn main_move() {
    let say_hi = {
        let name = String::from("Alice");
        move || println!("Hello, {}", name)
    };
    call_fn(&say_hi);call_fn(&say_hi);call_fn(&say_hi);
    call_fn_mut(&say_hi);call_fn_mut(&say_hi);call_fn_mut(&say_hi);
    call_fn_once(&say_hi);call_fn_once(&say_hi);call_fn_once(&say_hi);
}


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
    I: Iterator,
    I::Item: std::ops::Mul<Output = I::Item> + From<u8> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map_or_else(|| None, |n| Some(n*From::from(2u8)))
    }
}

fn do_sum<I>(a:I, b:I) -> I 
    where
    I: std::ops::Add<Output = I>{
    a+b
}

fn do_sum_fancy<I>(iter: I) -> I::Item
    where
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + From<u8>,
{
    iter.fold(From::from(0u8), std::ops::Add::add)
}

struct InfiniteUnit;

impl IntoIterator for InfiniteUnit{
    type Item=();

    type IntoIter = InfiniteUnitIter;

    fn into_iter(self) -> Self::IntoIter {
        InfiniteUnitIter
    }
}

struct InfiniteUnitIter;

impl Iterator for InfiniteUnitIter{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}

struct InfiniteUnit2;

impl IntoIterator for InfiniteUnit2 {
    type Item=();

    type IntoIter=std::iter::Repeat<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat(())
    }
}

fn main_iterators() {
    let mut count = 0;
    for _ in InfiniteUnit {
        count += 1;
        println!("count == {}", count);
        if count >= 5 {
            break;
        }
    }

    let mut count2 = 0;
    for _ in InfiniteUnit2 {
        count2 += 1;
        println!("count2 == {}", count2);
        if count2 >= 5 {
            break;
        }
    }
}

fn main_iters() {
    for i in Empty(33).take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }

    for i in one_to_ten() {
        println!("The answer to life, the universe, and everything is {}", i);
    }

    for i in fibo().take(10) {
        println!("fibo {}", i);
    }

    let orig = 1u32..10; // array indices start at 1
    let double_iter = Doubler {
        iter: orig,
    };
    for i in double_iter {
        println!("double {:?}", i);
    }

    for i in (1..11).map(|x| x*2) {
        println!("doblesillo {}", i);
    }

    for i in (1..11).skip(3).map(|x| x+1).filter(|x| x%2==0) {
        println!("skip map filter {}", i);
    }

    // collect things
    let c: Vec<i32> = (1..11).collect();
    println!("collect {c:?}");

    let s = (1..11).fold(100, |x,y| x+y);
    println!("x+y={s:?}");

    let s = (1..11).fold(100, |x,y| do_sum(x, y));
    println!("sum={s:?}");

    let s = do_sum_fancy(1..11);
    println!("sum fancy={s:?}");

    let res = (1..11).fold(0, std::ops::Add::add);
    println!("std sum{}", res);

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
