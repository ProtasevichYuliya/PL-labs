extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

struct StraightLine {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

struct LinearEquation{
    a: f64,
    b: f64,
    c: f64,
}

struct Point{
    x: f64,
    y: f64,
}

fn parse_line(line: &String) -> StraightLine {
    let re = Regex:: new(r"([\d\.]+),([\d\.]+) ([\d\.]+),([\d\.]+)").unwrap();
    let cap = re.captures(line).unwrap();
    StraightLine{
        x1: cap[1].parse::<f64>().unwrap(),
        y1: cap[2].parse::<f64>().unwrap(),
        x2: cap[3].parse::<f64>().unwrap(),
        y2: cap[4].parse::<f64>().unwrap(),
    } 
}

fn build_equation(straight_line: &StraightLine) -> LinearEquation{
    LinearEquation{
        a: straight_line.y1 - straight_line.y2,
        b: straight_line.x2 - straight_line.x1,
        c: straight_line.x1 * straight_line.y2 - straight_line.x2 * straight_line.y1,
    }
}

fn find_intersection(equation1: &LinearEquation, equation2: &LinearEquation) -> Point {
    if equation2.a != 0.0 {
        let y = (equation1.a * equation2.c / equation2.a - equation1.c) / (equation1.b - (equation2.b * equation1.a / equation2.a)); 
        Point{
            y: y,
            x: (-equation2.b * y - equation2.c) / equation2.a,
        }
    }
    else {
        let x = (equation1.b * equation2.c / equation2.b - equation1.c) / (equation1.a - (equation1.b * equation2.a / equation2.b));
        Point{
            x: x,
            y: (-equation2.a * x - equation2.c) / equation2.b, 
        }
    }
}

fn point_in_segment(point: &Point, segment: &StraightLine) -> bool {
    if (point.x <= f64::max(segment.x1, segment.x2) && point.x >= f64::min(segment.x1, segment.x2)) && (point.y <= f64::max(segment.y1, segment.y2) && point.y >= f64::min(segment.y1, segment.y2)){
        true
    }
    else {
         false
    }
}

fn point_in_ray(point: &Point, ray: &StraightLine) -> bool {
    if ((ray.x2 - ray.x1) * (point.x - ray.x1)) > 0.0 && ((ray.y2 - ray.y1) * (point.y - ray.y1)) > 0.0 {
        true
    }
    else {
        false
    }
}

fn distance_to_point(ray: &StraightLine, point: &Point) -> f64 {
    ((point.x - ray.x1) * (point.x - ray.x1) + (point.y - ray.y1) * (point.y - ray.y1)).sqrt()
}

fn main() {
    let input = File::open("input.txt").expect("File not found");
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    reader.read_line(&mut buffer).expect("Unable to read");
    let ray = parse_line(&buffer);
    let ray_eq = build_equation(&ray);
    buffer.clear();

    let mut segments: Vec<(StraightLine, Point, f64)> = Vec::new();
    while reader.read_line(&mut buffer).expect("Unable to read") > 0 {
        let line_segment = parse_line(&buffer);
        let segment_eq = build_equation(&line_segment);

        let intersection = find_intersection(&ray_eq, &segment_eq);
        if point_in_segment(&intersection, &line_segment) && point_in_ray (&intersection, &ray){
            // println!("intersects in ({}, {})", intersection.x, intersection.y);
            let distance = distance_to_point(&ray, &intersection);
            segments.push((line_segment, intersection, distance));
        }
        // else {
        //     println!("no intersection!");
        // }

        buffer.clear();
    }

    if segments.len() > 0 {
        segments.sort_by(|first, second| first.2.partial_cmp(&second.2).unwrap());
        let closest = segments.first().unwrap();
        println!("{},{} {},{}", closest.0.x1, closest.0.y1, closest.0.x2, closest.0.y2)
    } else {
        println!("\n")
    }
}