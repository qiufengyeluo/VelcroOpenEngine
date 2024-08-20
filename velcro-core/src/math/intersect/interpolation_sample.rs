#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]


enum  InterpolationMode
{
    NoInterpolation,
    LinearInterpolation,
}
pub trait InterpolationSample<Value>{
     fn get_interpolated_value(time:TimeType) ->Value;
}
type TimeType = u32;
pub struct Sample<Value>{
    _target_value:Value,
    _target_timestamp:TimeType,

    _previous_value:Value,
    _previous_timestamp:TimeType
}
impl <Value> InterpolationSample<Value> for Sample<Value>{
    #[inline]
    #[allow(dead_code)]
    fn get_interpolated_value(time:TimeType) ->Value{

    }
}
impl<Value> Sample<Value>{

    #[inline]
    #[allow(dead_code)]
     fn new()->Sample<Value>{
        Sample{
            _target_value:Value::new(),
            _target_timestamp:0,
            _previous_value:Value::new(),
            _previous_timestamp:0
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_new_target(&mut self, new_value:&Value, timestamp:&TimeType){
        self._target_value = new_value.to_owned();
        self._target_timestamp = timestamp.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_target_value(self)->Value{
        return self._target_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_target_timestamp(self)->TimeType{
        return self._target_timestamp;
    }

    #[inline]
    #[allow(dead_code)]
    fn set_previous_value(&mut self, previous_value:&Value, previous_timestamp:&TimeType){
        self._previous_value = previous_value.to_owned();
        self._previous_timestamp = previous_timestamp.to_owned();
    }

}