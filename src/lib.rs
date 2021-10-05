use std::ops::Add;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

// Vec3f
#[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Index<i32> for Vec3f {
    type Output = f64;

    fn index(&self, ind: i32) -> &Self::Output {
        match ind {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &0_f64,
        }
    }
}
impl Mul for Vec3f {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Neg for Vec3f {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Add for Vec3f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Default for Vec3f {
    fn default() -> Self {
        Vec3f {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        }
    }
}

impl Vec3f {
    pub fn norm(&self) -> f64 {
        (*self * *self).sqrt()
    }
    pub fn normalize(&self) -> Vec3f {
        let r: f64 = self.norm();
        if r == 0_f64 {
            return Vec3f::default();
        }
        Vec3f {
            x: self.x / r,
            y: self.y / r,
            z: self.z / r,
        }
    }
    pub fn mul_num(self, rhs: f64) -> Vec3f {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
    pub fn copy_in(&mut self, other: Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }
}

// Vec4f
#[derive(Copy, Clone)]
pub struct Vec4f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: f64,
}

impl Index<i32> for Vec4f {
    type Output = f64;

    fn index(&self, ind: i32) -> &Self::Output {
        match ind {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.t,
            _ => &0_f64,
        }
    }
}

impl Vec4f {
    pub fn copy_in(&mut self, other: Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
        self.t = other.t;
    }
}

// Sphere
impl Sphere {
    pub fn ray_intersect(self, orig: &Vec3f, dir: &Vec3f, dist: &mut f64) -> bool {
        let cur_dir: Vec3f = self.center - *orig;
        let cur_dir_projection_len: f64 = cur_dir * (*dir);
        let dist2: f64 = cur_dir * cur_dir - cur_dir_projection_len * cur_dir_projection_len;
        if dist2 > self.radius * self.radius {
            return false;
        }
        let thc: f64 = (self.radius * self.radius - dist2).sqrt();
        *dist = cur_dir_projection_len - thc;
        if *dist < 1e-3 {
            *dist = cur_dir_projection_len + thc;
        }
        if *dist < 1e-3 {
            return false;
        }
        true
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f64,
    pub material: Material,
}

// Material
#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub albedo: Vec4f,
    pub specular_exponent: f64,
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_color: Vec3f::default(),
            albedo: Vec4f {
                x: 1_f64,
                y: 0_f64,
                z: 0_f64,
                t: 0_f64,
            },
            specular_exponent: 0_f64,
            refractive_index: 1_f64,
        }
    }
}

impl Material {
    pub fn copy_in(&mut self, other: Self) {
        self.diffuse_color.copy_in(other.diffuse_color);
        self.albedo.copy_in(other.albedo);
        self.specular_exponent = other.specular_exponent;
        self.refractive_index = other.refractive_index;
    }
}

// Light
pub struct Light {
    pub position: Vec3f,
    pub intensity: f64,
}
