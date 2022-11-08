use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Displacement, Position};
use num_traits::Float;
use std::ops::Range;
use std::sync::{Arc, Weak};

pub(crate) struct HitRecord<F: Float, const N: usize> {
    pub(crate) p: Position<F, N>,
    pub(crate) normal: Displacement<F, N>,
    pub(crate) material: Weak<dyn Material<F, N> + Send + Sync>,
    t: F,
    pub(crate) front_face: bool,
}

impl<F: Float, const N: usize> HitRecord<F, N> {
    pub(crate) fn new(
        p: Position<F, N>,
        t: F,
        material: &Arc<dyn Material<F, N> + Send + Sync>,
    ) -> Self {
        Self {
            p,
            normal: Displacement::new(),
            material: Arc::downgrade(material),
            t,
            front_face: false,
        }
    }

    pub(crate) fn set_face_normal(&mut self, ray: &Ray<F, N>, outward_normal: &Displacement<F, N>) {
        self.front_face = Displacement::dot(&ray.direction, outward_normal) < F::zero();
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub(crate) trait Hittable<F: Float, const N: usize> {
    fn hit_by(&self, ray: &Ray<F, N>, t_range: Range<F>) -> Option<HitRecord<F, N>>;
}

pub(crate) struct HittableList<F: Float, const N: usize> {
    pub(crate) objects: Vec<Arc<dyn Hittable<F, N> + Send + Sync>>,
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
