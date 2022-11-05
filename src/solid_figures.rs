use num_traits::Float;
use std::ops::Range;
use std::rc::Rc;
use crate::vector::*;
use crate::ray::*;

pub type Vector3<T> = Vector<T, 3>;

impl<F: Float> Vector3<F> {
    pub fn new() -> Self {
        Vector::from_arr([F::zero(); 3])
    }
    pub fn cross(&self, other: &Self) -> Self {
        Vector::from_arr([
            self[1] * other[2] - self[2] - other[1],
            self[2] * other[0] - self[0] - other[2],
            self[0] * other[1] - self[1] - other[0],
            ])
    }
}

pub type Ray3<F> = Ray<F, 3>;

pub struct HitRecord<F: Float> {
    p: Vector3<F>,
    pub normal: Vector3<F>,
    t: F,
    front_face: bool,
}

impl<F: Float> HitRecord<F> {
    fn new(p: Vector3<F>, t: F) -> Self { Self { p, normal: Vector3::new(), t, front_face: false } }

    fn set_face_normal(&mut self, ray: &Ray3<F>, outward_normal: &Vector3<F>) {
        self.front_face = Vector::dot(&ray.direction, outward_normal) < F::zero();
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable<F: Float> {
    fn hit_by(&self, ray: &Ray3<F>, t_range: Range<F>) -> Option<HitRecord<F>>;
}

pub struct Sphere<F: Float> {
    center: Vector3<F>,
    radius: F,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Vector3<F>, radius: F) -> Self {
        Self { center, radius }
    }
}

impl<F> Hittable<F> for Sphere<F> where F: Float {
    fn hit_by(&self, ray: &Ray3<F>, t_range: Range<F>) -> Option<HitRecord<F>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_pow2();
        let half_b = Vector::dot(&oc, &ray.direction);
        let c = oc.norm_pow2() - self.radius.powi(2);

        let half_discriminant = half_b.powi(2) - a * c;
        if half_discriminant < F::zero() {
            return None;
        }
        let sqrt_half_discriminant = half_discriminant.sqrt();

        let mut root = (-half_b - sqrt_half_discriminant) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrt_half_discriminant) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let mut rec = HitRecord::new(p, t);
        let outward_normal = (p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }
}

pub struct HittableList<F: Float> {
    pub objects: Vec<Rc<dyn Hittable<F>>>,
}

impl<F: Float> Hittable<F> for HittableList<F> {
    fn hit_by(&self, ray: &Ray3<F>, t_range: Range<F>) -> Option<HitRecord<F>> {
        let mut ans = None;
        let mut closest_so_far = t_range.end;

        for object in &self.objects {
            if let Some(rec) = object.hit_by(ray, t_range.start..closest_so_far) {
                closest_so_far = rec.t;
                ans = Some(rec)
            }
        }

        ans
    }
}