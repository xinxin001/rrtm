use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};
use std::{f64::consts::PI, sync::Arc};

#[derive(Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Option<Arc<dyn Material>>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        return Self {
            center: Ray::new(static_center, Vec3::default()),
            radius: f64::max(0., radius),
            material: Some(material),
            bbox: AABB::with_points(&(static_center - rvec), &(static_center + rvec)),
        };
    }
    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::with_points(&(center.at(0.) - rvec), &(center.at(0.) + rvec));
        let box2 = AABB::with_points(&(center.at(1.) - rvec), &(center.at(1.) + rvec));
        return Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            material: Some(material),
            bbox: AABB::with_boxes(&box1, &box2),
        };
    }

    /// p: given a point on the sphere of radius one, centered at the origin
    /// u: returned value [0,1] of angle around the Y-axis from X=1
    /// v: returned value [0,1] of angle from Y=-1 to Y=+10
    /// <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_sphere(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        *u = phi / (2. * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin(); // C - Q
        let a = r.direction().length_squared(); // d * d
        let h = dot(r.direction(), oc); // simplified b, b = -2h
        let c = oc.length_squared() - self.radius * self.radius; // (C-Q)*(C-Q) - radius^2
        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return false;
        }

        // Here we are computing the full quadratic equation
        // We are checking if the resulting 't' falls inside the accepted interval
        let sqrtd = f64::sqrt(discriminant);

        // Check if root falls in acceptable range. Check for both signs of the root
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        // We update the hitrecord with the 't', point of intersect
        // and the unit-length of the intersect surface normal
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.material = self.material.clone();
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Self::get_sphere(&outward_normal, &mut rec.u, &mut rec.v);
        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn hit_sphere_naive(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2. * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return (-b - f64::sqrt(discriminant)) / (2. * a);
    }
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return (h - f64::sqrt(discriminant)) / a;
    }
}
