#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::sphere::Sphere;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug,Copy, Clone)]
pub struct Hemisphere{
     _centerRadius:Vector4,
     _direction:Vector3
}

impl PartialEq<Self> for Hemisphere {
    fn eq(&self, other: &Self) -> bool {
        unsafe {  return (self._centerRadius == other._centerRadius) && (self._direction == other._direction); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !(self == other); }
    }
}

impl Hemisphere{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Hemisphere{
        Hemisphere{
            _direction:Vector3::new(),
            _centerRadius:Vector4::new()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3_f32_vec3(center:&Vector3, radius:f32, normalized_direction:&Vector3) ->Hemisphere{
        Hemisphere{
            _direction: normalized_direction.to_owned(),
            _centerRadius:Vector4::new_vec3_w(center,radius)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_sphere_and_direction(sphere:&Sphere, normalized_direction:&Vector3) ->Hemisphere{
        return Hemisphere::new_vec3_f32_vec3(sphere.get_center().borrow(), sphere.get_radius(), normalized_direction);
    }
    static Hemisphere CreateFromSphereAndDirection(const Sphere& sphere, const Vector3& normalizedDirection);

    Vector3 GetCenter() const;
    float GetRadius() const;
    const Vector3& GetDirection() const;
    void SetCenter(const Vector3& center);
    void SetRadius(float radius);
    void SetDirection(const Vector3& direction);
}