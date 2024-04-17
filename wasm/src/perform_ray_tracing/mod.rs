use std::f64::consts::PI;

use super::rng;
use super::sphere::{Ray, Refl, Sphere};
use super::vec::Vec as Vec3;

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/console.js")]
extern "C" {
    fn log(count: usize) -> usize;
}

fn radiance(spheres: &[Sphere], r: Ray, mut depth: i32) -> Vec3 {
    fn intersect(spheres: &[Sphere], r: Ray) -> (bool, usize, f64) {
        let inf = 1e20;
        let mut t: f64 = 1e20;
        let mut id: usize = 0;

        for (i, sphere) in spheres.iter().enumerate() {
            let d = sphere.intersect(r);
            if d != 0. && d < t {
                t = d;
                id = i;
            }
        }

        return (t < inf, id, t);
    }

    let inte = intersect(spheres, r);

    if inte.0 == false {
        return Vec3::zero();
    }
    let id = inte.1;
    let t = inte.2;

    let obj = &spheres[id];
    let x: Vec3 = r.o + r.d * t;
    let n: Vec3 = (x - obj.p).norm();
    let nl = if (n & r.d) < 0. { n * 1. } else { n * (-1.) };
    let mut f = obj.c;
    let p = if f.x > f.y && f.x > f.z {
        f.x
    } else if f.y > f.z {
        f.y
    } else {
        f.z
    };

    let mut rng = rng::RngWrap::new();

    depth += 1;

    if depth > 5 {
        let g = rng.gen();
        if g < p {
            f = f * (1. / p);
        } else {
            return obj.e;
        }
    }

    if obj.refl == Refl::DIFF {
        let r1 = 2. * PI * rng.gen();
        let r2 = rng.gen();
        let r2s = r2.sqrt();
        let w: Vec3 = nl;
        let u: Vec3 = ((if w.x.abs() > 0.1 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        }) % w)
            .norm();
        let v = w % u;

        let d: Vec3 = (u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1. - r2).sqrt()).norm();
        return obj.e + f * radiance(spheres, Ray::new(x, d), depth);
    } else if obj.refl == Refl::SPEC {
        return obj.e + f * radiance(spheres, Ray::new(x, r.d - n * 2. * (n & r.d)), depth);
    }

    let refl_ray = Ray::new(x, r.d - n * 2. * (n & r.d));
    let into = n & nl > 0.;
    let nc = 1.;
    let nt = 1.5;
    let nnt = if into { nc / nt } else { nt / nc };
    let ddn = r.d & nl;
    let cos2t = 1. - nnt * nnt * (1. - ddn * ddn);
    if cos2t < 0. {
        return obj.e + f * radiance(spheres, refl_ray, depth);
    }

    let tdir =
        (r.d * nnt - (n * (if into { 1. } else { -1. })) * (ddn * nnt + cos2t.sqrt())).norm();

    let a = nt - nc;
    let b = nt + nc;
    let r0 = a * a / (b * b);
    let c = 1. - (if into { -ddn } else { tdir & n });

    let re = r0 + (1. - r0) * c * c * c * c * c;
    let tr = 1. - re;
    let p = 0.25 + 0.5 * re;
    let rp = re / p;
    let tp = tr / (1. - p);

    let res = obj.e
        + f * if depth > 2 {
            if rng.gen() < p {
                radiance(spheres, refl_ray, depth) * rp
            } else {
                radiance(spheres, Ray::new(x.clone(), tdir), depth) * tp
            }
        } else {
            radiance(spheres, refl_ray, depth) * re
                + radiance(spheres, Ray::new(x, tdir), depth) * tr
        };

    return res;
}

pub fn perform_ray_tracing(w: usize, h: usize, samps: u32, y: usize) -> Vec<Vec3> {
    fn clamp(x: f64) -> f64 {
        if x < 0. {
            0.
        } else if x > 1. {
            1.
        } else {
            x
        }
    }

    let spheres = [
        // 1
        Sphere::new(
            1e5,
            Vec3::new(1e5 + 1., 40.8, 81.6),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.75, 0.25, 0.25),
            Refl::DIFF,
        ),
        // 2
        Sphere::new(
            1e5,
            Vec3::new(-1e5 + 99., 40.8, 81.6),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.25, 0.25, 0.75),
            Refl::DIFF,
        ),
        // 3
        Sphere::new(
            1e5,
            Vec3::new(50., 40.8, 1e5),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.75, 0.75, 0.75),
            Refl::DIFF,
        ),
        // 4
        Sphere::new(
            1e5,
            Vec3::new(50., 40.8, -1e5 + 170.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 0., 0.),
            Refl::DIFF,
        ),
        // 5
        Sphere::new(
            1e5,
            Vec3::new(50., 1e5, 81.6),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.75, 0.75, 0.75),
            Refl::DIFF,
        ),
        // 6
        Sphere::new(
            1e5,
            Vec3::new(50., -1e5 + 81.6, 81.6),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.75, 0.75, 0.75),
            Refl::DIFF,
        ),
        // 7
        Sphere::new(
            16.5,
            Vec3::new(27., 16.5, 47.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0.999, 0.999, 0.999),
            Refl::SPEC,
        ),
        // 8
        Sphere::new(
            16.5,
            Vec3::new(73., 16.5, 78.),
            Vec3::zero(),
            Vec3::new(0.999, 0.999, 0.999),
            Refl::REFR,
        ),
        // 9
        Sphere::new(
            600.,
            Vec3::new(50., 681.6 - 0.27, 81.6),
            Vec3::new(12., 12., 12.),
            Vec3::zero(),
            Refl::DIFF,
        ),
    ];

    let cam = Ray::new(
        Vec3::new(50., 52., 295.6),
        Vec3::new(0., -0.042612, -1.).norm(),
    );

    let cx = Vec3::new(w as f64 * 0.5135 / h as f64, 0., 0.);

    let cy = (cx % cam.d).norm() * 0.5135;

    let mut c: Vec<Vec3> = Vec::new();

    for _ in 0..w {
        c.push(Vec3::new(0., 0., 0.));
    }

    let mut rng = rng::RngWrap::new();

    for x in 0..w {
        for sy in 0..2 {
            // let i = (h - y - 1) * w + x;
            for sx in 0..2 {
                let mut r = Vec3::new(0., 0., 0.);
                for _ in 0..samps {
                    let r1 = 2. * rng.gen();
                    let dx = if r1 < 1. {
                        r1.sqrt() - 1.
                    } else {
                        1. - (2. - r1).sqrt()
                    };

                    let r2 = 2. * rng.gen();
                    let dy = if r2 < 1. {
                        r2.sqrt() - 1.
                    } else {
                        1. - (2. - r2).sqrt()
                    };

                    let d1 = cx * (((sx as f64 + 0.5 + dx) / 2. + x as f64) / w as f64 - 0.5);
                    let d2 = cy * (((sy as f64 + 0.5 + dy) / 2. + y as f64) / h as f64 - 0.5);

                    let d = d1 + d2 + cam.d;

                    let dr = radiance(&spheres, Ray::new(cam.o + d * 140., d.norm()), 0)
                        * (1. / (samps as f64));

                    r = r + dr;
                }

                c[x] = c.get(x).unwrap().clone()
                    + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)) * 0.25;
            }
        }
    }

    return c;
}
