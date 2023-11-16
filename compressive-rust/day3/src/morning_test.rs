#![allow(unused_variables, dead_code)]

use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read, Result, Write},
};

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

struct Fibonacci {
    cur: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.cur + self.next;
        self.cur = self.next;
        self.next = new_next;
        Some(self.cur)
    }
}

fn from_iter_collect() {
    let primes = vec![2, 3, 5, 7];
    // FromIterator collect
    let prime_squares = primes.into_iter().map(|x| x * x).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}

fn from_into_example() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s}, {addr}, {one}, {bigger}");

    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}

// Read & BufRead
fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

// Drop 类死于析构函数
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn drop_example() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");
}

// Default or #[derive(Debug, Default)]

