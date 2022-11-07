use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Displacement, Position, Vector};
use num_traits::Float;
use std::ops::Range;
use std::rc::{Rc, Weak};

pub struct HitRecord<F: Float, const N: usize> {
    pub p: Position<F, N>,
    pub normal: Displacement<F, N>,
    pub material: Weak<dyn Material<F, N>>,
    t: F,
    front_face: bool,
}

impl<F: Float, const N: usize> HitRecord<F, N> {
    pub(crate) fn new(p: Position<F, N>, t: F, material: &Rc<dyn Material<F, N>>) -> Self {
        Self {
            p,
            normal: Displacement::new(),
            material: Rc::downgrade(material),
            t,
            front_face: false,
        }
    }

    pub(crate) fn set_face_normal(&mut self, ray: &Ray<F, N>, outward_normal: &Displacement<F, N>) {
        self.front_face = Vector::dot(&ray.direction, outward_normal) < F::zero();
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable<F: Float, const N: usize> {
    fn hit_by(&self, ray: &Ray<F, N>, t_range: Range<F>) -> Option<HitRecord<F, N>>;
}

pub struct HittableList<F: Float, const N: usize> {
    pub objects: Vec<Rc<dyn Hittable<F, N>>>,
}

impl<F: Float, const N: usize> Hittable<F, N> for HittableList<F, N> {
    fn hit_by(&self, ray: &Ray<F, N>, t_range: Range<F>) -> Option<HitRecord<F, N>> {
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
