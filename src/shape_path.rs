// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Color, with_current_cc};
use lyon_path::math::point;
use lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use sokol::gl as sgl;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

enum Command {
    MoveTo(Point),
    LineTo(Point),
    BezierTo(Point, Point, Point),
    QuadraticTo(Point, Point),
    CurveVertex(Point),
}

struct PathState {
    commands: Vec<Command>,
    closed: bool,
}

static PATH_STATE: LazyLock<Mutex<Option<PathState>>> = LazyLock::new(|| Mutex::new(None));

pub fn begin_shape() {
    begin_shape_kind();
}

pub fn begin_shape_kind() {
    *PATH_STATE.lock().unwrap() = Some(PathState {
        commands: Vec::new(),
        closed: false,
    });
}

pub fn vertex(x: f32, y: f32) {
    push_command(Command::LineTo(Point { x, y }));
}

pub fn move_to(x: f32, y: f32) {
    push_command(Command::MoveTo(Point { x, y }));
}

pub fn bezier_vertex(cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
    push_command(Command::BezierTo(
        Point { x: cx1, y: cy1 },
        Point { x: cx2, y: cy2 },
        Point { x, y },
    ));
}

pub fn quadratic_vertex(cx: f32, cy: f32, x: f32, y: f32) {
    push_command(Command::QuadraticTo(Point { x: cx, y: cy }, Point { x, y }));
}

pub fn curve_vertex(x: f32, y: f32) {
    push_command(Command::CurveVertex(Point { x, y }));
}

pub fn bezier(x1: f32, y1: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x2: f32, y2: f32) {
    begin_shape();
    move_to(x1, y1);
    bezier_vertex(cx1, cy1, cx2, cy2, x2, y2);
    end_shape(false);
}

pub fn curve(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
    begin_shape();
    curve_vertex(x1, y1);
    curve_vertex(x2, y2);
    curve_vertex(x3, y3);
    curve_vertex(x4, y4);
    end_shape(false);
}

pub fn begin_contour() {
    move_to(f32::NAN, f32::NAN);
}

pub fn end_contour() {}

pub fn end_shape(close: bool) {
    let Some(mut state) = PATH_STATE.lock().unwrap().take() else {
        return;
    };
    state.closed = close;
    let Some((color, is_filled, resolution)) = with_current_cc(|cc| {
        (
            cc.current_style.color,
            cc.current_style.fill,
            cc.current_style.curve_resolution.max(1) as usize,
        )
    }) else {
        return;
    };

    let subpaths = flatten(&state.commands, state.closed, resolution);
    draw_path(&subpaths, color, is_filled);
}

pub fn triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
    begin_shape();
    move_to(x1, y1);
    vertex(x2, y2);
    vertex(x3, y3);
    end_shape(true);
}

pub fn arc(x: f32, y: f32, width: f32, height: f32, start: f32, stop: f32) {
    let resolution =
        with_current_cc(|cc| cc.current_style.circle_resolution.max(3) as usize).unwrap_or(32);
    let steps = ((stop - start).abs() / std::f32::consts::TAU * resolution as f32)
        .ceil()
        .max(2.0) as usize;
    begin_shape();
    move_to(x, y);
    for index in 0..=steps {
        let t = index as f32 / steps as f32;
        let angle = start + (stop - start) * t;
        vertex(
            x + width * 0.5 * angle.cos(),
            y + height * 0.5 * angle.sin(),
        );
    }
    end_shape(true);
}

fn push_command(command: Command) {
    if let Some(state) = PATH_STATE.lock().unwrap().as_mut() {
        state.commands.push(command);
    }
}

fn flatten(commands: &[Command], close: bool, resolution: usize) -> Vec<Vec<Point>> {
    let mut paths = Vec::new();
    let mut current = Vec::new();
    let mut index = 0;
    while index < commands.len() {
        match commands[index] {
            Command::MoveTo(point) if point.x.is_nan() => {
                if !current.is_empty() {
                    paths.push(std::mem::take(&mut current));
                }
            }
            Command::MoveTo(point) => {
                if !current.is_empty() {
                    paths.push(std::mem::take(&mut current));
                }
                current.push(point);
            }
            Command::LineTo(point) => current.push(point),
            Command::BezierTo(control_1, control_2, end) => {
                let Some(&start) = current.last() else {
                    index += 1;
                    continue;
                };
                for step in 1..=resolution {
                    let t = step as f32 / resolution as f32;
                    current.push(cubic(start, control_1, control_2, end, t));
                }
            }
            Command::QuadraticTo(control, end) => {
                let Some(&start) = current.last() else {
                    index += 1;
                    continue;
                };
                for step in 1..=resolution {
                    let t = step as f32 / resolution as f32;
                    current.push(quadratic(start, control, end, t));
                }
            }
            Command::CurveVertex(_) => {
                let start = index;
                while index < commands.len() && matches!(commands[index], Command::CurveVertex(_)) {
                    index += 1;
                }
                let points: Vec<Point> = commands[start..index]
                    .iter()
                    .filter_map(|command| match command {
                        Command::CurveVertex(point) => Some(*point),
                        _ => None,
                    })
                    .collect();
                if points.len() >= 4 {
                    current.push(points[1]);
                    for window in points.windows(4) {
                        let p0 = window[0];
                        let p1 = window[1];
                        let p2 = window[2];
                        let p3 = window[3];
                        let c1 = add(p1, scale(sub(p2, p0), 1.0 / 6.0));
                        let c2 = sub(p2, scale(sub(p3, p1), 1.0 / 6.0));
                        for step in 1..=resolution {
                            let t = step as f32 / resolution as f32;
                            current.push(cubic(p1, c1, c2, p2, t));
                        }
                    }
                }
                continue;
            }
        }
        index += 1;
    }
    if !current.is_empty() {
        paths.push(current);
    }
    if close {
        for path in &mut paths {
            if let Some(first) = path.first().copied() {
                if path
                    .last()
                    .map(|last| distance(*last, first) > 0.001)
                    .unwrap_or(false)
                {
                    path.push(first);
                }
            }
        }
    }
    paths.retain(|path| path.len() >= 2);
    paths
}

fn draw_path(paths: &[Vec<Point>], color: Color, is_filled: bool) {
    if paths.is_empty() {
        return;
    }
    sgl::c4f(color.r, color.g, color.b, color.a);
    if is_filled {
        let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
        let mut builder = lyon_path::Path::builder();
        for path in paths {
            if path.len() < 3 {
                continue;
            }
            builder.begin(point(path[0].x, path[0].y));
            for vertex in &path[1..] {
                builder.line_to(point(vertex.x, vertex.y));
            }
            builder.end(true);
        }
        let path = builder.build();
        let mut tessellator = FillTessellator::new();
        if tessellator
            .tessellate_path(
                &path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    let position = vertex.position();
                    [position.x, position.y]
                }),
            )
            .is_ok()
        {
            sgl::begin_triangles();
            for index in geometry.indices {
                let vertex = geometry.vertices[index as usize];
                sgl::v2f(vertex[0], vertex[1]);
            }
            sgl::end();
        }
    } else {
        for path in paths {
            sgl::begin_line_strip();
            for point in path {
                sgl::v2f(point.x, point.y);
            }
            sgl::end();
        }
    }
}

fn cubic(p0: Point, p1: Point, p2: Point, p3: Point, t: f32) -> Point {
    let one_minus_t = 1.0 - t;
    Point {
        x: one_minus_t.powi(3) * p0.x
            + 3.0 * one_minus_t.powi(2) * t * p1.x
            + 3.0 * one_minus_t * t.powi(2) * p2.x
            + t.powi(3) * p3.x,
        y: one_minus_t.powi(3) * p0.y
            + 3.0 * one_minus_t.powi(2) * t * p1.y
            + 3.0 * one_minus_t * t.powi(2) * p2.y
            + t.powi(3) * p3.y,
    }
}

fn quadratic(p0: Point, p1: Point, p2: Point, t: f32) -> Point {
    let one_minus_t = 1.0 - t;
    Point {
        x: one_minus_t.powi(2) * p0.x + 2.0 * one_minus_t * t * p1.x + t.powi(2) * p2.x,
        y: one_minus_t.powi(2) * p0.y + 2.0 * one_minus_t * t * p1.y + t.powi(2) * p2.y,
    }
}

fn add(left: Point, right: Point) -> Point {
    Point {
        x: left.x + right.x,
        y: left.y + right.y,
    }
}

fn sub(left: Point, right: Point) -> Point {
    Point {
        x: left.x - right.x,
        y: left.y - right.y,
    }
}

fn scale(point: Point, factor: f32) -> Point {
    Point {
        x: point.x * factor,
        y: point.y * factor,
    }
}

fn distance(left: Point, right: Point) -> f32 {
    ((left.x - right.x).powi(2) + (left.y - right.y).powi(2)).sqrt()
}
