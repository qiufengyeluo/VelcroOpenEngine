#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ptr::null;

use crate::math::aabb::Aabb;
use crate::math::math_utils::constants::is_close_f32;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::VertexContainer::VertexContainer;

#[derive(Debug, Copy, Clone)]
pub struct SplineAddress{
    _segment_index:u64,
    _segment_fraction:f32,
}
impl PartialEq<Self> for SplineAddress {
    fn eq(&self, other: &Self) -> bool {
        unsafe {  return (self._segment_index == other._segment_index) && is_close_f32(self._segment_fraction, other._segment_fraction, SplineAddress::S_SEGMENT_FRACTION_EPSILON); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !(self == other); }
    }
}
impl SplineAddress{
    const S_SEGMENT_FRACTION_EPSILON: f32 = 0.0;
    pub fn new_index(segment_index:u64) ->SplineAddress{
        SplineAddress{
            _segment_index: segment_index,
            _segment_fraction:0.0,
        }
    }
    pub fn new(segment_index:u64, segment_fraction:f32) ->SplineAddress{
        SplineAddress{
            _segment_index: segment_index,
            _segment_fraction: segment_fraction,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PositionSplineQueryResult{
    _distance_sq:f32,
    _spline_address:SplineAddress,
}
impl PositionSplineQueryResult{
    pub fn new(spline_address:&SplineAddress, distance_sq:f32) ->PositionSplineQueryResult{
        PositionSplineQueryResult{
            _spline_address: spline_address.to_owned(),
            _distance_sq: distance_sq,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct RaySplineQueryResult{
    _ray_distance:f32,
    _position_spline_query_result:PositionSplineQueryResult,
}
impl RaySplineQueryResult  {
    pub fn new(spline_address:&SplineAddress, distance_sq:f32, ray_distance:f32) ->RaySplineQueryResult{
        RaySplineQueryResult{
            _ray_distance: ray_distance,
            _position_spline_query_result:PositionSplineQueryResult::new(spline_address, distance_sq)
        }
    }
}

trait SplineType{
    fn get_nearest_address_ray(self,local_ray_src:&Vector3, local_ray_dir:&Vector3) ->RaySplineQueryResult;
    fn get_nearest_address_position(self, local_pos:&Vector3) ->PositionSplineQueryResult;

    fn get_address_by_distance(self,distance:f32 ) ->SplineAddress;
    fn get_address_by_fraction(self,fraction:f32 ) ->SplineAddress;
    fn get_position(self, spline_address:& SplineAddress ) ->Vector3;
    fn get_normal(self, spline_address:& SplineAddress) ->Vector3;
    fn get_tangent(self, spline_address: &SplineAddress) ->Vector3;
    fn get_length(self, spline_address:&SplineAddress ) ->f32;
    fn get_spline_length(self) ->f32;
    fn get_segment_length(self,index:u32 ) ->f32;
    fn get_segment_count(self) ->u32;
    fn get_segment_granularity(self) ->u16;
    fn get_aabb(self, aabb:&Aabb, transform:&Transform);
}

#[derive(Debug, Copy, Clone)]
pub struct Spline{
    _closed:bool,
    _onOpenCloseCallback:VertexContainer::BoolFunction,
    _vertex_container:VertexContainer<Vector3>,
}

impl Spline{
    const S_SPLINE_EPSILON:f32 = 0.0;

    pub fn new()->Spline{
        Spline{
            _closed:false,
            _onOpenCloseCallback:null(),
            _vertex_container:VertexContainer::new(|index:usize|{ Spline::on_vertex_added(index); },
                                                   |index:usize|{Spline::on_vertex_removed(index);},
                                                   |index:usize|{Spline::on_spline_changed();},
                                                   ||{Spline::on_vertices_set()},
                                                   || { Spline::on_vertices_cleared(); }),
        }
    }

    pub fn set_closed(&mut self,closed:bool){
        if (closed != self._closed)
        {
            self._closed = closed;

            if (self._onOpenCloseCallback)
            {
                self._onOpenCloseCallback(closed);
            }
        }
    }

    pub fn is_closed(self)->bool{
        self._closed
    }

    pub fn get_vertex_count(self)->usize{
        self._vertex_container.size()
    }

    pub fn get_vertices(self) ->&'static Vec<Vector3>{
        self._vertex_container.get_vertices()
    }

    pub fn get_vertex(self, index:usize) ->&'static Vector3{
        &self._vertex_container.get_vertices()[index]
    }

    pub fn set_callbacks_change(&mut self, on_change_element:&VertexContainer::VoidFunction, on_change_container:& VertexContainer::VoidFunction,
                                on_open_close:&VertexContainer::BoolFunction ){
        self._vertex_container.set_callbacks(|index:usize|{
                                                Self::on_vertex_added(index);
                                                if on_change_container {
                                                    on_change_container();
                                                }
                                            },
                                             |index:usize|{
                                                 Self::on_vertex_removed(index);
                                                 if on_change_container {
                                                     on_change_container();
                                                 }
                                             },
                                             |index:usize|{
                                                 Self::on_spline_changed();
                                                 if on_change_element {
                                                     on_change_element();
                                                 }
                                             },
                                             ||{
                                                 Self::on_vertices_set();
                                                 if on_change_container {
                                                     on_change_container();
                                                 }
                                             },
                                             ||{
                                                 Self::on_vertices_cleared();
                                                 if on_change_container {
                                                     on_change_container();
                                                 }
                                             }
            );

        self._onOpenCloseCallback = on_open_close;
    }

    pub fn set_callbacks(&mut self, on_add_vertex:&VertexContainer::IndexFunction, on_remove_vertex:&VertexContainer::IndexFunction,
                         on_update_vertex:&VertexContainer::IndexFunction, on_set_vertices:&VertexContainer::VoidFunction,
                         on_clear_vertices:&VertexContainer::VoidFunction, on_open_close:&VertexContainer::BoolFunction){
        self._vertex_container.set_callbacks(
            |index:usize|{
                Self::on_vertex_added(index);
                if on_add_vertex {
                    on_add_vertex(index);
                }
            },
            |index:usize|{
                Self::on_vertex_removed(index);
                if on_remove_vertex {
                    on_remove_vertex(index);
                }
            },
            |index:usize|{
                Self::on_spline_changed();
                if on_update_vertex {
                    on_update_vertex(index);
                }
            },
            ||{
                Self::on_vertices_set();
                if on_set_vertices {
                    on_set_vertices();
                }
            },
            ||{
                Self::on_vertices_cleared();
                if on_clear_vertices {
                    on_clear_vertices();
                }
            }
            );

        self._onOpenCloseCallback = on_open_close;
    }

    pub fn on_spline_changed(){

    }

    pub fn on_vertex_added(index: usize){

    }
    pub fn on_vertices_set(){

    }
    pub fn on_vertex_removed(index: usize){

    }
    pub fn on_vertices_cleared(){

    }

    fn on_open_close_changed(self){
        if (self._onOpenCloseCallback)
        {
            self._onOpenCloseCallback(self._closed);
        }

        Self::OnSplineChanged();
    }

}
#[derive(Debug, Copy, Clone)]
pub struct LinearSpline{

}

impl LinearSpline:Spline{
LinearSpline()
: Spline() {}
explicit LinearSpline(const LinearSpline& spline)
: Spline(spline) {}
explicit LinearSpline(const Spline& spline)
: Spline(spline) {}
~LinearSpline() override {}

RaySplineQueryResult GetNearestAddressRay(const Vector3& localRaySrc, const Vector3& localRayDir) const override;
PositionSplineQueryResult GetNearestAddressPosition(const Vector3& localPos) const override;
SplineAddress GetAddressByDistance(float distance) const override;
SplineAddress GetAddressByFraction(float fraction) const override;

Vector3 GetPosition(const SplineAddress& splineAddress) const override;
Vector3 GetNormal(const SplineAddress& splineAddress) const override;
Vector3 GetTangent(const SplineAddress& splineAddress) const override;
float GetLength(const SplineAddress& splineAddress) const override;

float GetSplineLength() const override;
float GetSegmentLength(size_t index) const override;
size_t GetSegmentCount() const override;
void GetAabb(Aabb& aabb, const Transform& transform = Transform::CreateIdentity()) const override;

LinearSpline& operator=(const LinearSpline& spline) = default;
LinearSpline& operator=(const Spline& spline);

static void Reflect(SerializeContext& context);

protected:
u16 GetSegmentGranularity() const override { return 1; }
}

pub struct BezierSpline{

}
impl BezierSpline:Spline{

BezierSpline()
: Spline() {}
explicit BezierSpline(const BezierSpline& spline)
: Spline(spline)
, m_bezierData(spline.m_bezierData.begin(), spline.m_bezierData.end())
, m_granularity(spline.m_granularity) {}
explicit BezierSpline(const Spline& spline);
~BezierSpline() override {}

RaySplineQueryResult GetNearestAddressRay(const Vector3& localRaySrc, const Vector3& localRayDir) const override;
PositionSplineQueryResult GetNearestAddressPosition(const Vector3& localPos) const override;
SplineAddress GetAddressByDistance(float distance) const override;
SplineAddress GetAddressByFraction(float fraction) const override;

Vector3 GetPosition(const SplineAddress& splineAddress) const override;
Vector3 GetNormal(const SplineAddress& splineAddress) const override;
Vector3 GetTangent(const SplineAddress& splineAddress) const override;
float GetLength(const SplineAddress& splineAddress) const override;

float GetSplineLength() const override;
float GetSegmentLength(size_t index) const override;
size_t GetSegmentCount() const override;
void GetAabb(Aabb& aabb, const Transform& transform = Transform::CreateIdentity()) const override;

BezierSpline& operator=(const BezierSpline& spline);
BezierSpline& operator=(const Spline& spline);

static void Reflect(SerializeContext& context);

/**
 * Internal Bezier spline data
 */
struct BezierData
{
    AZ_TYPE_INFO(BezierData, "{6C34069E-AEA2-44A2-877F-BED9CE07DA6B}")
    AZ_CLASS_ALLOCATOR_DECL

    static void Reflect(SerializeContext& context);

    Vector3 m_back; ///< Control point before Vertex.
Vector3 m_forward; ///< Control point after Vertex.
float m_angle = 0.0f;
};

/**
 * Return immutable bezier data for each vertex
 */
const AZStd::vector<BezierData>& GetBezierData() const { return m_bezierData; }

protected:
void OnVertexAdded(size_t index) override;
void OnVerticesSet() override;
void OnVertexRemoved(size_t index) override;
void OnSplineChanged() override;
void OnVerticesCleared() override;

u16 GetSegmentGranularity() const override { return m_granularity; }

private:
/**
 * Functions to calculate bezier control points from input vertices
 */
void BezierAnglesCorrection(size_t index);
void BezierAnglesCorrectionRange(size_t index, size_t range);
void CalculateBezierAngles(size_t startIndex, size_t range, size_t iterations);

/**
 * Create bezier data for a given index
 */
void AddBezierDataForIndex(size_t index);
AZStd::vector<BezierData> m_bezierData; ///< Bezier data to control spline interpolation.
u16 m_granularity = 8; ///< The granularity (tessellation) of each segment in the spline.
}

pub struct CatmullRomSpline{
    _knot_parameterization:f32,
    _granularity:u16,
}

impl CatmullRomSpline{

    pub fn new()->CatmullRomSpline{
        CatmullRomSpline{
            _knot_parameterization:0.0,
            _granularity:8,

        }
    }
    CatmullRomSpline()
    : Spline() {}
    explicit CatmullRomSpline(const CatmullRomSpline& spline)
    : Spline(spline)
    , m_knotParameterization(spline.m_knotParameterization)
    , m_granularity(spline.m_granularity) {}
    explicit CatmullRomSpline(const Spline& spline);
    ~CatmullRomSpline() override {}

RaySplineQueryResult GetNearestAddressRay(const Vector3& localRaySrc, const Vector3& localRayDir) const override;
PositionSplineQueryResult GetNearestAddressPosition(const Vector3& localPos) const override;
SplineAddress GetAddressByDistance(float distance) const override;
SplineAddress GetAddressByFraction(float fraction) const override;

Vector3 GetPosition(const SplineAddress& splineAddress) const override;
Vector3 GetNormal(const SplineAddress& splineAddress) const override;
Vector3 GetTangent(const SplineAddress& splineAddress) const override;
float GetLength(const SplineAddress& splineAddress) const override;

float GetSplineLength() const override;
float GetSegmentLength(size_t index) const override;
size_t GetSegmentCount() const override;
void GetAabb(Aabb& aabb, const Transform& transform = Transform::CreateIdentity()) const override;

CatmullRomSpline& operator=(const CatmullRomSpline& spline) = default;
CatmullRomSpline& operator=(const Spline& spline);

/**
 * Sets the knot parameterization for spline interpolation, ranging from [0,1].
 * 0 = uniform; 0.5 = centriperal; 1 = chordal
 * @param knotParameterization value between [0, 1] to control spline interpolation
 */
void SetKnotParameterization(float knotParameterization) { m_knotParameterization = GetClamp(knotParameterization, 0.0f, 1.0f); }

static void Reflect(SerializeContext& context);

protected:
u16 GetSegmentGranularity() const override { return m_granularity; }


}

pub unsafe  fn intersect_spline(world_from_local:&Transform, src:&Vector3, dir:&Vector3, spline:&Spline) ->RaySplineQueryResult{
    let mut world_from_local_normalized = world_from_local.to_owned();
    let scale = world_from_local_normalized.extract_uniform_scale();
    let local_from_world_normalized = world_from_local_normalized.get_inverse();

    let local_ray_origin = local_from_world_normalized.transform_point_vec3(src) / scale;
    let local_ray_direction = local_from_world_normalized.transform_point_vec3(dir);
    return spline.GetNearestAddressRay(local_ray_origin, local_ray_direction);
}
