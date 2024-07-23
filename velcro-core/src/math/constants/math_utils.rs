#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

pub(crate) const PI:f32 = std::f32::consts::PI;
pub(crate) const TWO_PI:f32 = 6.28318530717958647692;
pub(crate) const HALF_PI:f32 = std::f32::consts::FRAC_PI_2;
pub(crate) const QUARTER_PI:f32 = std::f32::consts::FRAC_PI_4;
pub(crate) const TWO_OVER_PI:f32 = std::f32::consts::FRAC_2_PI;
pub(crate) const MAX_FLOAT_BEFORE_PRECISION_LOSS:f32 = 100000.0;
pub(crate) const TOLERANCE:f32 = 0.001;
pub(crate) const FLOAT_MAX:f32 = 3.402823466e+38;
pub(crate) const FLOAT_EPSILON:f32 = 1.192092896e-07;




const G_SIN_COEF1:[f32;4] = [ -0.0001950727, -0.0001950727, -0.0001950727, -0.0001950727 ];
const G_SIN_COEF2:[f32;4] = [0.0083320758,  0.0083320758,  0.0083320758,  0.0083320758 ];
const G_SIN_COEF3:[f32;4] =[-0.1666665247, -0.1666665247, -0.1666665247, -0.1666665247];
const G_COS_COEF1:[f32;4] = [-0.0013602249, -0.0013602249, -0.0013602249, -0.0013602249 ];
const G_COS_COEF2:[f32;4] = [0.0416566950,  0.0416566950,  0.0416566950,  0.0416566950];
const G_COS_COEF3:[f32;4] =[-0.4999990225, -0.4999990225, -0.4999990225, -0.4999990225];
const G_ACOS_HI_COEF1:[f32;4] = [ -0.0012624911, -0.0012624911, -0.0012624911, -0.0012624911 ];
const G_ACOS_HI_COEF2:[f32;4] = [  0.0066700901,  0.0066700901,  0.0066700901,  0.0066700901 ];
const G_ACOS_HI_COEF3:[f32;4] = [ -0.0170881256, -0.0170881256, -0.0170881256, -0.0170881256 ];
const G_ACOS_HI_COEF4:[f32;4] = [  0.0308918810,  0.0308918810,  0.0308918810,  0.0308918810 ];
const G_ACOS_LO_COEF1:[f32;4] = [ -0.0501743046, -0.0501743046, -0.0501743046, -0.0501743046 ];
const G_ACOS_LO_COEF2:[f32;4] = [  0.0889789874,  0.0889789874,  0.0889789874,  0.0889789874 ];
const G_ACOS_LO_COEF3:[f32;4] = [ -0.2145988016, -0.2145988016, -0.2145988016, -0.2145988016 ];
const G_ACOS_LO_COEF4:[f32;4] = [  1.5707963050,  1.5707963050,  1.5707963050,  1.5707963050 ];
const G_ACOS_COEF1:[f32;4]   = [ -0.0200752200, -0.0200752200, -0.0200752200, -0.0200752200 ];
const G_ACOS_COEF2:[f32;4]   = [  0.0759031500,  0.0759031500,  0.0759031500,  0.0759031500 ];
const G_ACOS_COEF3:[f32;4]   = [ -0.2126757000, -0.2126757000, -0.2126757000, -0.2126757000 ];
const G_ATAN_HI_RANGE:[f32;4] = [  2.4142135624,  2.4142135624,  2.4142135624,  2.4142135624 ];
const G_ATAN_LO_RANGE:[f32;4] = [  0.4142135624,  0.4142135624,  0.4142135624,  0.4142135624 ];
const G_ATAN_COEF1:[f32;4]   = [  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2 ];
const G_ATAN_COEF2:[f32;4]   = [ -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1 ];
const G_ATAN_COEF3:[f32;4]   = [  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1 ];
const G_ATAN_COEF4:[f32;4]   = [ -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1 ];
const G_EXP_COEF1:[f32;4]    = [  1.2102203e7, 1.2102203e7, 1.2102203e7, 1.2102203e7 ];
const G_EXP_COEF2:[i32;4]  = [ -8388608, -8388608, -8388608, -8388608];
const G_EXP_COEF3:[f32;4]    = [  1.1920929e-7, 1.1920929e-7, 1.1920929e-7, 1.1920929e-7 ];
const G_EXP_COEF4:[f32;4]    = [  3.371894346e-1, 3.371894346e-1, 3.371894346e-1, 3.371894346e-1 ];
const G_EXP_COEF5:[f32;4]    = [  6.57636276e-1, 6.57636276e-1, 6.57636276e-1, 6.57636276e-1 ];
const G_EXP_COEF6:[f32;4]    = [  1.00172476, 1.00172476, 1.00172476, 1.00172476 ];


 const G_VEC1111:[f32;4]         = [ 1.0 , 1.0 , 1.0 , 1.0 ];
 const G_VEC1000:[f32;4]         = [ 1.0 , 0.0 , 0.0 , 0.0 ];
 const G_VEC0100:[f32;4]         = [ 0.0 , 1.0 , 0.0 , 0.0 ];
 const G_VEC0010:[f32;4]         = [ 0.0 , 0.0 , 1.0 , 0.0 ];
 const G_VEC0001:[f32;4]         = [ 0.0 , 0.0 , 0.0 , 1.0 ];
 const G_PI:[f32;4]              = [ Constants::Pi, Constants::Pi, Constants::Pi, Constants::Pi ];
 const G_TWO_PI:[f32;4]           = [ Constants::TwoPi, Constants::TwoPi, Constants::TwoPi, Constants::TwoPi ];
 const G_HALF_PI:[f32;4]          = [ Constants::HalfPi, Constants::HalfPi, Constants::HalfPi, Constants::HalfPi ];
 const G_QUARTER_PI:[f32;4]       = [ Constants::QuarterPi, Constants::QuarterPi, Constants::QuarterPi, Constants::QuarterPi ];
 const G_TWO_OVER_PI:[f32;4]       = [ Constants::TwoOverPi, Constants::TwoOverPi, Constants::TwoOverPi, Constants::TwoOverPi ];
const G_ABS_MASK:[i32;4]       = [ 0x7ffffff , 0x7ffffff , 0x7ffffff , 0x7fffffff ];
const G_NEGATE_MASK:[i32;4]    = [  0x80000000,  0x80000000,  0x80000000,  0x80000000 ];
const G_NEGATE_XMASK:[i32;4]   = [  0x80000000,  0x00000000,  0x00000000,  0x00000000 ];
const G_NEGATE_YMASK:[i32;4]   = [  0x00000000,  0x80000000,  0x00000000,  0x00000000 ];
const G_NEGATE_ZMASK:[i32;4]   = [  0x00000000,  0x00000000,  0x80000000,  0x00000000 ];
const G_NEGATE_WMASK:[i32;4]   = [  0x00000000,  0x00000000,  0x00000000,  0x80000000 ];
const G_NEGATE_XYZMASK:[i32;4] = [  0x80000000,  0x80000000,  0x80000000,  0x00000000 ];
const G_W_MASK:[i32;4]         = [  0xfffffff ,  0xfffffff ,  0xfffffff ,  0x00000000 ];