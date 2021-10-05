use std::f64::consts::PI;
use tinyraytracer::{Light, Material, Sphere, Vec3f, Vec4f};

fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    lights: &[Light],
    depth: usize,
) -> Vec3f {
    let mut point = Vec3f::default();
    let mut surface_normal: Vec3f = Vec3f::default();
    let mut material: Material = Material::default();

    if depth > 4
        || !scene_intersect(
            orig,
            dir,
            spheres,
            &mut point,
            &mut surface_normal,
            &mut material,
        )
    {
        return Vec3f {
            x: 0.2,
            y: 0.7,
            z: 0.8,
        };
    }

    let reflect_dir = reflect(dir, &surface_normal).normalize();
    let refract_dir = refract(dir, &surface_normal, &material.refractive_index).normalize();
    let reflect_orig = if reflect_dir * surface_normal < 0.0 {
        point - surface_normal.mul_num(1e-3)
    } else {
        point + surface_normal.mul_num(1e-3)
    };
    let refract_orig = if refract_dir * surface_normal < 0.0 {
        point - surface_normal.mul_num(1e-3)
    } else {
        point + surface_normal.mul_num(1e-3)
    };
    let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);
    let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in lights {
        let light_dir = (light.position - point).normalize();
        let light_distance = (light.position - point).norm();

        let shadow_orig = if light_dir * surface_normal < 0.0 {
            point - surface_normal.mul_num(1e-3)
        } else {
            point + surface_normal.mul_num(1e-3)
        };
        let mut shadow_pt: Vec3f = Vec3f::default();
        let mut shadow_n: Vec3f = Vec3f::default();
        let mut tmpmaterial: Material = Material::default();
        if scene_intersect(
            &shadow_orig,
            &light_dir,
            spheres,
            &mut shadow_pt,
            &mut shadow_n,
            &mut tmpmaterial,
        ) && (shadow_pt - shadow_orig).norm() < light_distance
        {
            continue;
        }

        diffuse_light_intensity += light.intensity * f64::max(0.0, light_dir * surface_normal);
        specular_light_intensity += f64::max(0.0, reflect(&light_dir, &surface_normal) * (*dir))
            .powf(material.specular_exponent)
            * light.intensity;
    }

    material
        .diffuse_color
        .mul_num(diffuse_light_intensity)
        .mul_num(material.albedo[0])
        + Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        .mul_num(specular_light_intensity * material.albedo[1])
        + reflect_color.mul_num(material.albedo[2])
        + refract_color.mul_num(material.albedo[3])
}

fn render(spheres: &[Sphere], lights: &[Light]) {
    let width = 1024;
    let height = 768;
    let fov = PI / 2.;
    let mut framebuffer: Vec<Vec3f> = Vec::new();

    for j in 0..height {
        for i in 0..width {
            let x: f64 = (2.0 * (i as f64 + 0.5) / (width as f64) - 1.0)
                * (fov / 2.0).tan()
                * (width as f64)
                / (height as f64);
            let y: f64 = -(2.0 * (j as f64 + 0.5) / (height as f64) - 1.0) * (fov / 2.0).tan();
            let dir: Vec3f = Vec3f { x, y, z: -1.0 }.normalize();
            framebuffer.push(cast_ray(&Vec3f::default(), &dir, spheres, lights, 0));
        }
    }

    // save the framebuffer to file
    use std::io::Write;
    let mut file = std::fs::File::create("./out.ppm").unwrap();
    let header =
        ("P6\n".to_string() + &*width.to_string() + " " + &*height.to_string() + "\n255\n")
            .into_bytes();

    file.write_all(&header).unwrap();
    for mut vec in framebuffer {
        let max = f64::max(vec[0], f64::max(vec[1], vec[2]));
        if max > 1.0 {
            vec = vec.mul_num(1.0 / max)
        }
        for j in 0..3 {
            let line = (255.0 * f64::max(0.0, f64::min(1.0, vec[j]))) as u8;
            file.write_all(&[line]).expect("File write failed.");
        }
    }
}

fn refract(incident_angle: &Vec3f, surface_normal: &Vec3f, refractive_index: &f64) -> Vec3f {
    // Snell's law
    let mut cos_incident_angle =
        -f64::max(-1.0, f64::min(1.0, (*incident_angle) * (*surface_normal)));
    let mut etai = 1.0;
    let mut etat = *refractive_index;
    let mut surface_normal = *surface_normal;
    if cos_incident_angle < 0.0 {
        // if the ray is inside the object, swap the indices and invert the normal to get the correct result
        cos_incident_angle = -cos_incident_angle;
        std::mem::swap(&mut etai, &mut etat);
        surface_normal = -surface_normal;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cos_incident_angle * cos_incident_angle);
    if k < 0.0 {
        Vec3f::default()
    } else {
        (*incident_angle).mul_num(eta) + surface_normal.mul_num(eta * cos_incident_angle - k.sqrt())
    }
}

fn reflect(incident_angle: &Vec3f, surface_normal: &Vec3f) -> Vec3f {
    (*incident_angle) - surface_normal.mul_num(2.0 * ((*incident_angle) * (*surface_normal)))
}

fn scene_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
    hit: &mut Vec3f,
    surface_normal: &mut Vec3f,
    material: &mut Material,
) -> bool {
    let mut spheres_dist: f64 = 1000.0;
    for sphere in spheres {
        let mut dist_i: f64 = 0.0;
        if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
            spheres_dist = dist_i;
            hit.copy_in(*orig + dir.mul_num(dist_i));
            (*surface_normal).copy_in((*hit - sphere.center).normalize());
            (*material).copy_in(sphere.material);
        }
    }

    let mut checkerboard_dist = 1000.0;
    if dir.y.abs() > 1e-3 {
        let d = -((*orig).y + 4.0) / dir.y; // the checkerboard plane has equation y = -4
        let pt = *orig + dir.mul_num(d);
        if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10.0 && pt.z > -30.0 && d < spheres_dist {
            checkerboard_dist = d;
            hit.copy_in(pt);
            (*surface_normal).copy_in(Vec3f {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
            material.diffuse_color =
                if (((0.5 * hit.x + 1000.0) as i64) + ((0.5 * hit.z) as i64)) % 2 == 1 {
                    Vec3f {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                } else {
                    Vec3f {
                        x: 1.0,
                        y: 0.7,
                        z: 0.3,
                    }
                };
            material.diffuse_color = material.diffuse_color.mul_num(0.3);
        }
    }

    f64::min(spheres_dist, checkerboard_dist) < 1000.0
}

fn main() {
    let ivory: Material = Material {
        albedo: Vec4f {
            x: 0.6,
            y: 0.3,
            z: 0.1,
            t: 0.0,
        },
        diffuse_color: Vec3f {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        specular_exponent: 50.0,
        refractive_index: 1.0,
    };
    let glass: Material = Material {
        albedo: Vec4f {
            x: 0.0,
            y: 0.5,
            z: 0.1,
            t: 0.8,
        },
        diffuse_color: Vec3f {
            x: 0.6,
            y: 0.7,
            z: 0.8,
        },
        specular_exponent: 125.0,
        refractive_index: 1.5,
    };
    let red_rubber: Material = Material {
        albedo: Vec4f {
            x: 0.9,
            y: 0.1,
            z: 0.0,
            t: 0.0,
        },
        diffuse_color: Vec3f {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        specular_exponent: 10.0,
        refractive_index: 1.0,
    };
    let mirror: Material = Material {
        albedo: Vec4f {
            x: 0.0,
            y: 10.0,
            z: 0.8,
            t: 0.0,
        },
        diffuse_color: Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        specular_exponent: 1425.0,
        refractive_index: 1.0,
    };

    let spheres: Vec<Sphere> = vec![
        Sphere {
            center: Vec3f {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: ivory,
        },
        Sphere {
            center: Vec3f {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: glass,
        },
        Sphere {
            center: Vec3f {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 3.0,
            material: red_rubber,
        },
        Sphere {
            center: Vec3f {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 4.0,
            material: mirror,
        },
    ];

    let lights: Vec<Light> = vec![
        Light {
            position: Vec3f {
                x: -20.0,
                y: 20.0,
                z: 20.0,
            },
            intensity: 1.5,
        },
        Light {
            position: Vec3f {
                x: 30.0,
                y: 50.0,
                z: -25.0,
            },
            intensity: 1.8,
        },
        Light {
            position: Vec3f {
                x: 30.0,
                y: 20.0,
                z: 30.0,
            },
            intensity: 1.7,
        },
    ];
    render(&spheres, &lights);
}
