#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::plane::Plane;
use crate::math::vector4::Vector4;
use crate::math::vsimd::FloatType;

enum PlaneId
{
    Near,
    Far,
    Left,
    Right,
    Top,
    Bottom,
    MAX,
}

enum ReverseDepth
{
    True,
    False,
}

enum CornerIndices
{
    NearTopLeft,
    NearTopRight,
    NearBottomLeft,
    NearBottomRight,
    FarTopLeft,
    FarTopRight,
    FarBottomLeft,
    FarBottomRight,
    Count,
}

#[derive(Debug, Copy, Clone)]
pub struct Frustum {
    _planes:[FloatType;PlaneId::MAX as usize],
    _serialized_planes: [Plane;PlaneId::MAX as usize],
}

impl Frustum{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Frustum{
        Frustum{
            _planes:[FloatType;PlaneId::MAX as usize],
            _serialized_planes: [Plane;PlaneId::MAX as usize],
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_view_frustum_attributes(viewFrustumAttributes:&ViewFrustumAttributes)->Frustum{

    }
    Frustum::Frustum(const ViewFrustumAttributes& viewFrustumAttributes)
    {
    ConstructPlanes(viewFrustumAttributes);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_plane(near_plane:&Plane, far_plane:&Plane, left_plane:&Plane, right_plane:&Plane, top_plane:&Plane, bottom_plane:&Plane) ->Frustum{
        let mut result = Frustum::new();
        result.set_plane(PlaneId::Near.borrow(), near_plane);
        result.set_plane(PlaneId::Far.borrow(), far_plane);
        result.set_plane(PlaneId::Left.borrow(), left_plane);
        result.set_plane(PlaneId::Right.borrow(), right_plane);
        result.set_plane(PlaneId::Top.borrow(), top_plane);
        result.set_plane(PlaneId::Bottom.borrow(), bottom_plane);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_matrix_row_major(matrix:&Matrix4x4, reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }

        let frustum = Frustum::new();
        frustum.set_plane(near_plane_id.borrow(), Plane::create_from_vector_coefficients(matrix.get_column(2).borrow()));
        frustum.set_plane(far_plane_id.borrow(), Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(2)).borrow()));
        frustum.set_plane(PlaneId::Left.borrow(),   Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(0)).borrow()));
        frustum.set_plane(PlaneId::Right.borrow(),  Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(0)).borrow()));
        frustum.set_plane(PlaneId::Top.borrow(),    Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(1)).borrow()));
        frustum.set_plane(PlaneId::Bottom.borrow(), Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(1)).borrow()));
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_matrix_column_major(matrix:&Matrix4x4, reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }


        let frustum = Frustum::new();
        frustum.set_plane(near_plane_id.borrow(),     Plane::create_from_vector_coefficients(matrix.get_row(2)));
        frustum.set_plane(far_plane_id.borrow(),      Plane::create_from_vector_coefficients(matrix.get_row(3) - matrix.get_row(2)));
        frustum.set_plane(PlaneId::Left.borrow(),   Plane::create_from_vector_coefficients(matrix.get_row(3) + matrix.get_row(0)));
        frustum.set_plane(PlaneId::Right.borrow(),  Plane::create_from_vector_coefficients(matrix.get_row(3) - matrix.get_row(0)));
        frustum.set_plane(PlaneId::Top.borrow(),    Plane::create_from_vector_coefficients(matrix.get_row(3) - matrix.get_row(1)));
        frustum.set_plane(PlaneId::Bottom.borrow(), Plane::create_from_vector_coefficients(matrix.get_row(3) + matrix.get_row(1)));
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_matrix_row_major_symmetric_z(matrix:&Matrix4x4,reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }

        let frustum = Frustum::new();
        frustum.set_plane(near_plane_id.borrow(),     Plane::create_from_vector_coefficients(matrix.get_column(3) + matrix.get_column(2)));
        frustum.set_plane(far_plane_id.borrow(),      Plane::create_from_vector_coefficients(matrix.get_column(3) - matrix.get_column(2)));
        frustum.set_plane(PlaneId::Left.borrow(),   Plane::create_from_vector_coefficients(matrix.get_column(3) + matrix.get_column(0)));
        frustum.set_plane(PlaneId::Right.borrow(),  Plane::create_from_vector_coefficients(matrix.get_column(3) - matrix.get_column(0)));
        frustum.set_plane(PlaneId::Top.borrow(),    Plane::create_from_vector_coefficients(matrix.get_column(3) - matrix.get_column(1)));
        frustum.set_plane(PlaneId::Bottom.borrow(), Plane::create_from_vector_coefficients(matrix.get_column(3) + matrix.get_column(1)));
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_matrix_column_major_symmetric_z(matrix:&Matrix4x4,reverseDepth:&ReverseDepth)->Frustum{
        const PlaneId nearPlaneId = (reverseDepth == ReverseDepth::True) ? PlaneId::Far  : PlaneId::Near;
        const PlaneId farPlaneId  = (reverseDepth == ReverseDepth::True) ? PlaneId::Near : PlaneId::Far;

        Frustum frustum;
        frustum.SetPlane(nearPlaneId,     Plane::CreateFromVectorCoefficients(matrix.GetRow(3) + matrix.GetRow(2)));
        frustum.SetPlane(farPlaneId,      Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(2)));
        frustum.SetPlane(PlaneId::Left,   Plane::CreateFromVectorCoefficients(matrix.GetRow(3) + matrix.GetRow(0)));
        frustum.SetPlane(PlaneId::Right,  Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(0)));
        frustum.SetPlane(PlaneId::Top,    Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(1)));
        frustum.SetPlane(PlaneId::Bottom, Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(1)));
        return frustum;
    }
    Frustum Frustum::CreateFromMatrixColumnMajorSymmetricZ(const Matrix4x4& matrix, ReverseDepth reverseDepth)
    {
    const PlaneId nearPlaneId = (reverseDepth == ReverseDepth::True) ? PlaneId::Far  : PlaneId::Near;
    const PlaneId farPlaneId  = (reverseDepth == ReverseDepth::True) ? PlaneId::Near : PlaneId::Far;

    Frustum frustum;
    frustum.SetPlane(nearPlaneId,     Plane::CreateFromVectorCoefficients(matrix.GetRow(3) + matrix.GetRow(2)));
    frustum.SetPlane(farPlaneId,      Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(2)));
    frustum.SetPlane(PlaneId::Left,   Plane::CreateFromVectorCoefficients(matrix.GetRow(3) + matrix.GetRow(0)));
    frustum.SetPlane(PlaneId::Right,  Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(0)));
    frustum.SetPlane(PlaneId::Top,    Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(1)));
    frustum.SetPlane(PlaneId::Bottom, Plane::CreateFromVectorCoefficients(matrix.GetRow(3) - matrix.GetRow(1)));
    return frustum;
    }


    void Frustum::Set(const Frustum& frustum)
    {
    for (PlaneId planeId = PlaneId::Near; planeId < PlaneId::MAX; ++planeId)
    {
    SetPlane(planeId, frustum.m_serializedPlanes[planeId]);
    }
    }


    void Frustum::ConstructPlanes(const ViewFrustumAttributes& viewFrustumAttributes)
    {
    const float tanHalfFov = std::tan(viewFrustumAttributes.m_verticalFovRadians * 0.5f);
    const float nearPlaneHalfHeight = tanHalfFov * viewFrustumAttributes.m_nearClip;
    const float nearPlaneHalfWidth = nearPlaneHalfHeight * viewFrustumAttributes.m_aspectRatio;

    const Vector3 translation = viewFrustumAttributes.m_worldTransform.GetTranslation();
    const Vector3 forward = viewFrustumAttributes.m_worldTransform.GetBasisY();
    const Vector3 right = viewFrustumAttributes.m_worldTransform.GetBasisX();
    const Vector3 up = viewFrustumAttributes.m_worldTransform.GetBasisZ();

    SetPlane(
    PlaneId::Near,
    Plane::CreateFromNormalAndPoint(forward, translation + (forward * viewFrustumAttributes.m_nearClip)));
    SetPlane(
    PlaneId::Far,
    Plane::CreateFromNormalAndPoint(-forward, translation + (forward * viewFrustumAttributes.m_farClip)));

    const Vector3 leftNormal =
    (right + forward * (nearPlaneHalfWidth / viewFrustumAttributes.m_nearClip)).GetNormalized();
    const Vector3 rightNormal =
    (-right + forward * (nearPlaneHalfWidth / viewFrustumAttributes.m_nearClip)).GetNormalized();

    SetPlane(PlaneId::Left, Plane::CreateFromNormalAndPoint(leftNormal, translation));
    SetPlane(PlaneId::Right, Plane::CreateFromNormalAndPoint(rightNormal, translation));

    const Vector3 topNormal =
    (-up + forward * (nearPlaneHalfHeight / viewFrustumAttributes.m_nearClip)).GetNormalized();
    const Vector3 bottomNormal =
    (up + forward * (nearPlaneHalfHeight / viewFrustumAttributes.m_nearClip)).GetNormalized();

    SetPlane(PlaneId::Top, Plane::CreateFromNormalAndPoint(topNormal, translation));
    SetPlane(PlaneId::Bottom, Plane::CreateFromNormalAndPoint(bottomNormal, translation));
    }


    ViewFrustumAttributes Frustum::CalculateViewFrustumAttributes() const
    {
    using Simd::Vec4;

    ViewFrustumAttributes viewFrustumAttributes;

    const Vector3 forward = Vector4(m_planes[PlaneId::Near]).GetAsVector3().GetNormalized();
    const Vector3 right =
    Vector4(Vec4::Sub(m_planes[PlaneId::Left], m_planes[PlaneId::Right])).GetAsVector3().GetNormalized();
    const Vector3 up =
    Vector4(Vec4::Sub(m_planes[PlaneId::Bottom], m_planes[PlaneId::Top])).GetAsVector3().GetNormalized();

    const Matrix3x3 orientation = Matrix3x3::CreateFromColumns(right, forward, up);
    const Plane bottom(m_planes[PlaneId::Bottom]);
    const Plane top(m_planes[PlaneId::Top]);
    const Plane left(m_planes[PlaneId::Left]);

    // solve a set of simultaneous equations to find the point that is contained within all planes
    // (note: this is not a transformation, it is simply using Matrix3x3 to perform some linear algebra)
    const Vector3 origin =
    -Matrix3x3::CreateFromRows(bottom.GetNormal(), top.GetNormal(), left.GetNormal()).GetInverseFull() *
    Vector3(bottom.GetDistance(), top.GetDistance(), left.GetDistance());

    viewFrustumAttributes.m_worldTransform = Transform::CreateFromMatrix3x3AndTranslation(orientation, origin);

    const float originDotForward = origin.Dot(forward);
    const float nearClip = -Vec4::SelectIndex3(m_planes[PlaneId::Near]) - originDotForward;

    viewFrustumAttributes.m_nearClip = nearClip;
    viewFrustumAttributes.m_farClip = Vec4::SelectIndex3(m_planes[PlaneId::Far]) - originDotForward;

    const float leftNormalDotForward = left.GetNormal().Dot(forward);
    const float frustumNearHeight =
    2.0f * nearClip * leftNormalDotForward / std::sqrt(1.0f - leftNormalDotForward * leftNormalDotForward);
    const float bottomNormalDotForward = bottom.GetNormal().Dot(forward);
    const float tanHalfFov =
    bottomNormalDotForward / std::sqrt(1.0f - bottomNormalDotForward * bottomNormalDotForward);
    const float frustumNearWidth = 2.0f * nearClip * tanHalfFov;

    viewFrustumAttributes.m_aspectRatio = frustumNearHeight / frustumNearWidth;
    viewFrustumAttributes.m_verticalFovRadians = 2.0f * std::atan(tanHalfFov);

    return viewFrustumAttributes;
}

bool Frustum::GetCorners(CornerVertexArray& corners) const
{
using ShapeIntersection::IntersectThreePlanes;

return
IntersectThreePlanes(GetPlane(Near), GetPlane(Top), GetPlane(Left), corners[NearTopLeft]) &&
IntersectThreePlanes(GetPlane(Near), GetPlane(Top), GetPlane(Right), corners[NearTopRight]) &&
IntersectThreePlanes(GetPlane(Near), GetPlane(Bottom), GetPlane(Left), corners[NearBottomLeft]) &&
IntersectThreePlanes(GetPlane(Near), GetPlane(Bottom), GetPlane(Right), corners[NearBottomRight]) &&
IntersectThreePlanes(GetPlane(Far), GetPlane(Top), GetPlane(Left), corners[FarTopLeft]) &&
IntersectThreePlanes(GetPlane(Far), GetPlane(Top), GetPlane(Right), corners[FarTopRight]) &&
IntersectThreePlanes(GetPlane(Far), GetPlane(Bottom), GetPlane(Left), corners[FarBottomLeft]) &&
IntersectThreePlanes(GetPlane(Far), GetPlane(Bottom), GetPlane(Right), corners[FarBottomRight])
;
}

inline Frustum::PlaneId operator++(Frustum::PlaneId& planeId)
{
planeId = (Frustum::PlaneId)(planeId + 1);
return planeId;
}

AZ_MATH_INLINE Frustum::Frustum()
{
#ifdef AZ_DEBUG_BUILD
for (PlaneId i = PlaneId::Near; i < PlaneId::MAX; ++i)
{
m_planes[i] = Simd::Vec4::Splat(std::numeric_limits<float>::signaling_NaN());
}
#endif
}

AZ_MATH_INLINE Plane Frustum::GetPlane(PlaneId planeId) const
{
return Plane(m_planes[planeId]);
}

AZ_MATH_INLINE void Frustum::SetPlane(PlaneId planeId, const Plane& plane)
{
using namespace Simd;
m_serializedPlanes[planeId] = plane;

//normalize the plane by dividing each element by the length of the normal
const Vec4::FloatType lengthSquared = Vec4::FromVec1(Vec3::Dot(plane.GetNormal().GetSimdValue(), plane.GetNormal().GetSimdValue()));
const Vec4::FloatType length = Vec4::Sqrt(lengthSquared);
m_planes[planeId] = Vec4::Div(plane.GetSimdValue(), length);
}

AZ_MATH_INLINE IntersectResult Frustum::IntersectSphere(const Vector3& center, float radius) const
{
bool intersect = false;

for (PlaneId i = PlaneId::Near; i < PlaneId::MAX; ++i)
{
const float distance = Simd::Vec1::SelectIndex0(Simd::Vec4::PlaneDistance(m_planes[i], center.GetSimdValue()));

if (distance < -radius)
{
return IntersectResult::Exterior;
}

intersect |= (fabsf(distance) < radius);
}

return intersect ? IntersectResult::Overlaps : IntersectResult::Interior;
}

AZ_MATH_INLINE IntersectResult Frustum::IntersectSphere(const Sphere& sphere) const
{
return IntersectSphere(sphere.GetCenter(), sphere.GetRadius());
}

AZ_MATH_INLINE IntersectResult Frustum::IntersectAabb(const Vector3& minimum, const Vector3& maximum) const
{
return IntersectAabb(Aabb::CreateFromMinMax(minimum, maximum));
}

AZ_MATH_INLINE IntersectResult Frustum::IntersectAabb(const Aabb& aabb) const
{
// Count the number of planes where the AABB is inside
uint32_t numInterior = 0;

for (PlaneId i = PlaneId::Near; i < PlaneId::MAX; ++i)
{
const Vector3 disjointSupport = aabb.GetSupport(-Vector3(Simd::Vec4::ToVec3(m_planes[i])));
const float   disjointDistance = Simd::Vec1::SelectIndex0(Simd::Vec4::PlaneDistance(m_planes[i], disjointSupport.GetSimdValue()));

if (disjointDistance < 0.0f)
{
return IntersectResult::Exterior;
}

// We now know the interior point we just checked passes the plane check..
// Check an exterior support point to determine whether or not the whole AABB is contained or if this is an intersection
const Vector3 intersectSupport = aabb.GetSupport(Vector3(Simd::Vec4::ToVec3(m_planes[i])));
const float   intersectDistance = Simd::Vec1::SelectIndex0(Simd::Vec4::PlaneDistance(m_planes[i], intersectSupport.GetSimdValue()));

if (intersectDistance >= 0.0f)
{
// If the whole AABB passes the plane check, increment the number of planes the AABB is 'interior' to
++numInterior;
}
}

// If the AABB is interior to all planes, we're contained, else we intersect
return (numInterior < PlaneId::MAX) ? IntersectResult::Overlaps : IntersectResult::Interior;
}

    #[inline]
    #[allow(dead_code)]
    pub fn is_close(self, rhs:&Frustum,tolerance:&f32)->bool{
        unsafe {
            return Vector4::new_float_type(self._planes[PlaneId::Near].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Near].borrow()).borrow(), tolerance)
                && Vector4::new_float_type(self._planes[PlaneId::Far].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Far].borrow()).borrow(), tolerance)
                && Vector4::new_float_type(self._planes[PlaneId::Left].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Left].borrow()).borrow(), tolerance)
                && Vector4::new_float_type(self._planes[PlaneId::Right].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Right].borrow()).borrow(), tolerance)
                && Vector4::new_float_type(self._planes[PlaneId::Top].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Top].borrow()).borrow(), tolerance)
                && Vector4::new_float_type(self._planes[PlaneId::Bottom].borrow()).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Bottom].borrow()).borrow(), tolerance);
        }
    }

}






