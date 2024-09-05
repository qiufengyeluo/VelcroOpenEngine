#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ops::*;
use std::ops::Add;

use crate::math::math_utils::constants;
use crate::math::vector2::Vector2;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Color{
    _color:Vector4
}

impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
        return self._color == other._color;
    }
    fn ne(&self, other: &Self) -> bool {
        return self._color != other._color;
    }
}

impl Sub for Color{
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color{
            _color:-self._color,
        }
    }
}



impl Add<&Color> for Color{
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color{
            _color:self._color + rhs._color,
        }
    }
}

impl Sub<&Color> for Color {
    type Output = Color;

    fn sub(self, rhs: &Color) -> Self::Output {
        Color{
            _color:self._color - rhs._color,
        }
    }
}

impl Mul<&Color> for Color{
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color{
            _color:self._color * rhs._color,
        }
    }
}

impl Div<&Color> for Color{
    type Output = Color;

    fn div(self, rhs: &Color) -> Self::Output {
        Color{
            _color:self._color / rhs._color,
        }
    }
}

impl Mul<f32> for Color{
    type Output = Color;

    fn mul(self, multiplier: f32) -> Self::Output {
        Color{
            _color: self._color * multiplier,
        }
    }
}

impl Div<f32> for Color{
    type Output = Color;

    fn div(self, multiplier: f32) -> Self::Output {
        Color{
            _color: self._color / multiplier,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self._color += rhs._color;
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self._color -= rhs._color;
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self._color *= rhs._color;
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self._color /= rhs._color;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self._color *= rhs;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self._color /= rhs;
    }
}

impl  Color{

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new()->Color{
        Color{
            _color:Vector4::new(),
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec4(source:&Vector4)->Color{
        Color{
            _color:source.to_owned(),
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2(source:&Vector2)->Color{
        Color{
            _color:Vector4::new_vec2(source),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3(source:&Vector3)->Color{
        Color{
            _color:Vector4::new_vec3(source),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_f32(rgba:f32)->Color{
        Color{
            _color:Vector4::new_x(rgba)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_rgba_f32(r:f32,g:f32,b:f32,a:f32)->Color{
        Color{
            _color:Vector4::new_x_y_z_w(r,g,b,a),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_rgba_u8(r:&u8,g:&u8,b:&u8,a:&u8)->Color{
        let mut result = Color::new();
        result.set_r8(r);
        result.set_g8(g);
        result.set_b8(b);
        result.set_a8(a);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_zero()->Color{
        Color{
            _color:Vector4::create_zero(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_one()->Color{
        let  result = Color::new_f32(1.0);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_rgba(r:&u8,g:&u8,b:&u8,a:&u8)->Color{
        return Color::new_rgba_u8(r,g,b,a);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_float4(values:*const f32)->Color{
        let mut result = Color::new();
        result.set_float4(values);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3(v:&Vector3)->Color{
        let mut result = Color::new();
        result.set_vec3(v);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3and_float(v:&Vector3,w:f32)->Color{
        let mut result = Color::new();
        result.set_vec3_f32(v,w);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_u32(r:&u8,g:&u8,b:&u8,a:&u8)->u32{
        return (a << 24) | (b << 16) | (g << 8) | r;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_float4(&self, values:*mut f32){
        self._color.store_to_float_4(values)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_r8(self)->u8{
        return (self._color.get_x() * 255.0) as u8
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_g8(self)->u8{
        return (self._color.get_y() * 255.0) as u8
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_b8(self)->u8{
        return (self._color.get_z() * 255.0) as u8
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_a8(self)->u8{
        return (self._color.get_w() * 255.0) as u8
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_r8(&self,r:&u8){
        self._color.set_x(((r.to_owned() as f32)*(1.0/255.0)))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_g8(&self,g:&u8){
        self._color.set_y(((g.to_owned() as f32)*(1.0/255.0)))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_b8(&self,b:&u8){
        self._color.set_z(((b.to_owned() as f32)*(1.0/255.0)))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_a8(&self,a:&u8){
        self._color.set_w(((a.to_owned() as f32)*(1.0/255.0)))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_r(self)->f32{
        return self._color.get_x();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_g(self)->f32{
        return self._color.get_y();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_b(self)->f32{
        return self._color.get_z();
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_a(self)->f32{
        return self._color.get_w();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_element(self,index:i32)->f32{
        return self._color.get_element(index)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_r(&mut self,r:f32){
        self._color.set_x(r)
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_g(&mut self,g:f32){
        self._color.set_y(g)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_b(&mut self,b:f32){
        self._color.set_z(b)
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_a(&mut self,a:f32){
        self._color.set_w(a)
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_f32(&mut self,x:f32){
        self._color.set_f32(x);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rgba_f32(&mut self,r:f32,g:f32,b:f32,a:f32){
        self._color.set_x_y_z_w(r,g,b,a);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_float4(&mut self,values:*const f32){
        self._color.set_float4(values)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3(&mut self,v:&Vector3){
        self._color.set_vec3(v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3_f32(&mut self,v:&Vector3,a:f32){
        self._color.set_vec3_f32(v,a);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(&mut self,index:i32,v:f32){
        self._color.set_element(index,v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_vector3(self)->Vector3{
        return self._color.get_as_vector3();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_vector4(self)->Vector4{
        return self._color;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_from_hsvradians(&mut self, mut hue_radians:f32, mut saturation:f32, mut value:f32){
        let alpha = self.get_a();
        saturation = constants::get_clamp(saturation,0.0,1.0);
        value = constants::get_clamp(value,0.0,1.0);
        hue_radians = hue_radians.fmodf(constants::TWO_PI);
        if (hue_radians.to_owned() < 0f32)
        {
            hue_radians = &(hue_radians + constants::TWO_PI);
        }
        let hue = (hue_radians / constants::deg_to_rad(60.0.borrow())).fmodf(6.0);
        let hue_sexant = hue as i32;
        let hue_sexant_remainder = hue - hue_sexant;

        let off_color = value * (1.0 - saturation);
        let falling_color = value * (1.0 - (saturation * hue_sexant_remainder));
        let rising_color = value * (1.0 - (saturation * (1.0 - hue_sexant_remainder)));
        match hue_sexant {
            0=>{
                self.set_rgba_f32(value, rising_color, off_color.borrow(), alpha.borrow())
            }
            1=>{
                self.set_rgba_f32(falling_color, value, off_color.borrow(), alpha.borrow())
            }
            2=>{
                self.set_rgba_f32(off_color.borrow(), value, rising_color, alpha.borrow())
            }
            3=>{
                self.set_rgba_f32(off_color.borrow(), falling_color, value, alpha.borrow())
            }
            4=>{
                self.set_rgba_f32(rising_color, off_color.borrow(), value, alpha.borrow())
            }
            5=>{
                self.set_rgba_f32(value, off_color.borrow(), falling_color, alpha.borrow())
            }
            _ => {}
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(&self, v:&Color, tolerance:f32)->bool{
        return self._color.is_close(v.get_as_vector4().borrow(),tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_zero(&self,tolerance:f32)->bool{
        return self.is_close(Color::create_zero().borrow(),tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_finite(self)->bool{
        return self._color.is_finite();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn vector3(self)->Vector3{
        return self._color.get_as_vector3();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn vector4(self)->Vector4{
        return self._color;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn to_u32(self)->u32{
        return Color::create_u32(self.get_r8().borrow(),self.get_g8().borrow(),self.get_b8().borrow(),self.get_a8().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn from_u32(&mut self, c:u32){
        self.set_a((((c.to_owned() >> 24) as f32) * (1.0 / 255.0) ));
        self.set_b((((c.to_owned() >> 16) & 0xff)as f32 * (1.0 / 255.0)));
        self.set_g((((c.to_owned() >> 8) & 0xff)as f32 * (1.0 / 255.0)));
        self.set_r((((c.to_owned() & 0xff)as f32) * (1.0 / 255.0)));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn to_u32linear_to_gamma(self)->u32{
        return self.linear_to_gamma().to_u32()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn from_u32gamma_to_linear(&mut self,c:u32){
        self.from_u32(c);
        self._color =  self.gamma_to_linear()._color;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_srgb_gamma_to_linear(x:f32) ->f32{
        if x.to_owned() <= 0.04045 {
            return x / 12.92;
        }
        return  ((x + 0.055) / 1.055).powf(2.4);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_srgb_linear_to_gamma(x:f32)->f32{
        if x.to_owned() <= 0.0031308 {
            return 12.92 * x
        }
        return  (1.055 * x.powf(1.0 / 2.4) - 0.055);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn linear_to_gamma(self) ->Color{
        let mut r = self.get_r();
        let mut g =  self.get_g();
        let mut b =  self.get_b();
        r = Color::convert_srgb_linear_to_gamma(r);
        g = Color::convert_srgb_linear_to_gamma(g);
        b = Color::convert_srgb_linear_to_gamma(b);
        return Color::new_rgba_f32(r,g,b,self.get_a()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn gamma_to_linear(self)->Color{
        let mut r = self.get_r();
        let mut g =  self.get_g();
        let mut b =  self.get_b();
        return Color::new_rgba_f32(Color::convert_srgb_gamma_to_linear(r),
                                   Color::convert_srgb_gamma_to_linear(g),
                                   Color::convert_srgb_gamma_to_linear(b),
                                    self.get_a())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_than(self,rhs:&Color)->bool{
        return self._color.is_less_than(rhs._color.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_equal_than(self,rhs:&Color)->bool{
        return self._color.is_less_equal_than(rhs._color.borrow());
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_greater_than(self,rhs:&Color)->bool{
        return  self._color.is_greater_than(rhs._color.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_greater_equal_than(self,rhs:&Color)->bool{
        return  self._color.is_greater_equal_than(rhs._color.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn lerp(self,dest:&Color,t:f32)->Color{
        return Color::new_vec4(self._color.lerp(dest._color.borrow(),t).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot(&mut self,rhs:&Color)->f32{
        return self._color.dot4(rhs._color.borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot3(&mut self,rhs:&Color)->f32{
        return self._color.dot3(rhs._color.get_as_vector3().borrow());
    }

}