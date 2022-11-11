use num_traits::Float;
use std::sync::Arc;
use std::ops::Range;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::solid_figures::{Displacement3, HitRecord3, Position3};

pub(crate) struct Cuboid<F: Float> {
    planes: [[F; 2]; 3],
    inward: bool,

    material: Arc<dyn Material<F, 3> + Send + Sync>,
}

impl<F: Float> Cuboid<F> {
    pub(crate) fn new(
        vertexes: [Position3<F>; 2],
        inward: bool,
        material: Arc<dyn Material<F, 3> + Send + Sync>,
    ) -> Self {
        Self {
            planes: [
                [vertexes[0].arr()[0], vertexes[1].arr()[0]],
                [vertexes[0].arr()[1], vertexes[1].arr()[1]],
                [vertexes[0].arr()[2], vertexes[1].arr()[2]],
            ],
            inward,
            material,
        }
    }
}

impl<F: Float> Hittable<F, 3> for Cuboid<F> {
    fn hit_by(&self, ray: &Ray<F, 3>, t_range: Range<F>) -> Option<HitRecord<F, 3>> {
        let mut roots = [[F::zero(); 2]; 3];
        for i in 0..3 {
            roots[i] = self.planes[i]
                .map(|plane| (plane - ray.origin.arr()[i]) / ray.direction.arr()[i]);
        }
        let mut fronts = [0; 3];
        let mut backs = [0; 3];
        for i in 0..3 {
            (fronts[i], backs[i]) = if roots[i][0] < roots[i][1] {
                (0, 1)
            } else {
                (1, 0)
            }
        }
        let front_axis = if roots[0][fronts[0]] < roots[1][fronts[1]] {
            if roots[1][fronts[1]] < roots[2][fronts[2]] {
                2
            } else {
                1
            }
        } else if roots[0][fronts[0]] < roots[2][fronts[2]] {
            2
        } else {
            0
        };
        let back_axis = if roots[0][backs[0]] > roots[1][backs[1]] {
            if roots[1][backs[1]] > roots[2][backs[2]] {
                2
            } else {
                1
            }
        } else if roots[0][backs[0]] > roots[2][backs[2]] {
            2
        } else {
            0
        };
        if roots[front_axis][fronts[front_axis]] > roots[back_axis][backs[back_axis]] {
            return None;
        }
        let (t, axis, negative) = if t_range.contains(&roots[front_axis][fronts[front_axis]]) {
            (
                roots[front_axis][fronts[front_axis]],
                front_axis,
                self.planes[front_axis][fronts[front_axis]]
                    < self.planes[front_axis][1 - fronts[front_axis]],
            )
        } else if t_range.contains(&roots[back_axis][backs[back_axis]]) {
            (
                roots[back_axis][backs[back_axis]],
                back_axis,
                self.planes[back_axis][backs[back_axis]]
                    < self.planes[back_axis][1 - backs[back_axis]],
            )
        } else {
            return None;
        };
        let p = ray.at(t);
        let mut rec = HitRecord3::new(p, t, &self.material);
        let mut outward_normal = Displacement3::new();
        outward_normal.arr_mut()[axis] = F::one();
        if negative {
            outward_normal = -outward_normal;
        }
        if self.inward {
            outward_normal = -outward_normal;
        }
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }
}
