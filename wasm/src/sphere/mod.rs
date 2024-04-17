use super::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub o: Vec,
  pub d: Vec,
}

impl Ray {
  pub fn new(o: Vec, d: Vec) -> Ray {
      Ray { o, d }
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Refl {
  DIFF,
  SPEC,
  REFR,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
  pub rad: f64,
  pub p: Vec,
  pub e: Vec,
  pub c: Vec,
  pub refl: Refl,
}

impl Sphere {
  pub fn new(rad: f64, p: Vec, e: Vec, c: Vec, refl: Refl) -> Sphere {
      Sphere { rad, p, e, c, refl }
  }

  pub fn intersect(&self, r: Ray) -> f64 {
      let op = self.p - r.o;
      let eps = 1e-4;
      let b = op & r.d;
      let mut det = b * b - (op & op) + self.rad * self.rad;
      if det < 0. {
          return 0.;
      }
      det = det.sqrt();
      let mut t = b - det;
      if t > eps {
          return t;
      }
      t = b + det;
      if t > eps {
          return t;
      }
      return 0.;
  }
}

#[cfg(test)]
mod tests;