#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate z3;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use z3::Ast;
use z3::Config;
use z3::Context;
use z3::Optimize;

#[derive(Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

fn parse_input(input: &str) -> Bot {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pos=<([-]?\d+),([-]?\d+),([-]?\d+)>, r=(\d+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let z = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let r = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
    Bot { x, y, z, r }
}

fn q1(bots: &mut Vec<Bot>) {
    let largest_r = bots.iter().map(|b| b.r).max().unwrap();
    let largest_r_bot_idx = bots.iter().position(|b| b.r == largest_r).unwrap();
    let mut q1_counter = 0;

    bots.iter().for_each(|b| {
        let md = (b.x - bots[largest_r_bot_idx].x).abs()
            + (b.y - bots[largest_r_bot_idx].y).abs()
            + (b.z - bots[largest_r_bot_idx].z).abs();
        if md <= largest_r {
            q1_counter += 1;
        }
    });
    println!("result of q01 is {}", q1_counter);
}

fn z3_abs<'ctx>(n: &Ast<'ctx>, ctx: &'ctx Context) -> Ast<'ctx> {
    n.ge(&ctx.from_i64(0)).ite(n, &n.minus())
}

fn z3_dist<'ctx>(t1: (&Ast<'ctx>, &Ast<'ctx>, &Ast<'ctx>),
                 t2: (i64, i64, i64), ctx: &'ctx Context) -> Ast<'ctx> {
    let tmp1 = &z3_abs(&t1.0.sub(&[&ctx.from_i64(t2.0)]), &ctx);
    let tmp2 = &z3_abs(&t1.1.sub(&[&ctx.from_i64(t2.1)]), &ctx);
    let tmp3 = &z3_abs(&t1.2.sub(&[&ctx.from_i64(t2.2)]), &ctx);
    ctx.from_i64(0).add(&[tmp1, tmp2, tmp3])
}

fn q2(bots: &Vec<Bot>) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = &ctx.named_int_const("x");
    let y = &ctx.named_int_const("y");
    let z = &ctx.named_int_const("z");
    let mut cost_expr = x.mul(&[&ctx.from_i64(0)]);

    bots.iter().for_each(|b| {
        let md_ast = z3_dist((x,y,z), (b.x, b.y, b.z), &ctx);
        let if_ast = &md_ast
            .le(&ctx.from_i64(b.r))
            .ite(&ctx.from_i64(1), &ctx.from_i64(0));
        cost_expr = cost_expr.add(&[if_ast]);
    });
    let opt = Optimize::new(&ctx);
    opt.maximize(&cost_expr);
    opt.minimize(&z3_dist((x,y,z), (0,0,0), &ctx));
    println!("{}", opt.check());
    let model = opt.get_model();
    let x_val = model.eval(&x).unwrap().as_i64().unwrap();
    let y_val = model.eval(&y).unwrap().as_i64().unwrap();
    let z_val = model.eval(&z).unwrap().as_i64().unwrap();
    println!("{} {} {}", x_val, y_val, z_val);
    println!("result of q02 is {}", x_val.abs() + y_val.abs() + z_val.abs());
}

fn main() {
    let path = format!("./input/{}", "day23.txt");

    let mut bots: Vec<Bot> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| parse_input(&s))
        .collect();

    q1(&mut bots);

    // remember `brew install z3`
    q2(&bots);
}
