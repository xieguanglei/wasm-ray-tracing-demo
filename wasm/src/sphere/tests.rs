use super::*;

#[test]
fn intersect() {
    let sp = Sphere::new(
        1.,
        Vec::new(5., 0., 0.),
        Vec::new(0., 0., 0.),
        Vec::new(0., 0., 0.),
        Refl::DIFF,
    );
    let r = Ray::new(
      Vec::new(0., 0., 0.), 
      Vec::new(1., 0., 0.)
    );
    let dis = sp.intersect(r);
    assert_eq!(dis, 4.);
}
