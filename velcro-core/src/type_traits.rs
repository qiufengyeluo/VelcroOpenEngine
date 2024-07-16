pub use velcro_derive::{VObjectProvider, TypeUuidProvider};
use std::any::{Any, TypeId};
use std::path::PathBuf;
use velcro_utils::UUID;



pub mod prelude {
    pub use super::{combine_uuids, VObjectProvider, TypeUuidProvider};
    pub use velcro_utils::UUID;
}

/// A trait for an entity that has unique type identifier.
pub trait TypeUuidProvider: Sized {
    /// Return type UUID.
    fn type_uuid() -> UUID;
}


#[macro_export]
macro_rules! uuid_provider {
    ($type:ident $(<$($generics:tt),*>)? = $uuid:expr) => {
        impl$(<$($generics),*>)? $crate::type_traits::TypeUuidProvider for $type $(<$($generics),*>)? {
            fn type_uuid() -> $crate::UUID {
                $crate::UUID::create_string($uuid)
            }
        }
    };
}


uuid_provider!(u8 = "{fe415306-719d-4e1e-8b0b-02ed9d572808}");
uuid_provider!(i8 = "{9e4eb3f1-40ad-4d15-98aa-77539bb7ccdd}");
uuid_provider!(u16 = "{9e532675-c35a-4d89-97e3-84bd238d3a39}");
uuid_provider!(i16 = "{7814cab9-38fa-4b57-af36-da8cc89324e5}");
uuid_provider!(u32 = "{dcd3d344-5584-43d7-812d-1cebd8e3f4d0}");
uuid_provider!(i32 = "{e006e268-b0ce-4c90-9b3b-2b885143aa59}");
uuid_provider!(u64 = "{125b2ab3-a6a2-441e-afc0-c8f991d4ee82}");
uuid_provider!(i64 = "{106873ac-ed93-42d3-b575-a0a05054ff83}");
uuid_provider!(f32 = "{38221112-ecdf-4de3-8e26-b03415d6ec71}");
uuid_provider!(f64 = "{a6c38158-c26a-43bd-a6ff-82defd0480da}");
uuid_provider!(usize = "{a00657de-4ae7-4bb4-b3bc-c801afcdda21}");
uuid_provider!(isize = "{8825747d-3725-41a2-960b-24b590bb0ced}");
uuid_provider!(bool = "{061c42ab-7d58-4899-8b67-1b9145a1413b}");
uuid_provider!(PathBuf = "{37a9b93d-2c96-4282-84d6-28a51467a2f5}");
uuid_provider!(String = "{7835cad2-beb2-4fcf-87af-b702b136becc}");

impl<T: TypeUuidProvider> TypeUuidProvider for Option<T> {
    fn type_uuid() -> UUID {
        combine_uuids(
            UUID::create_string("{f44d8e0c-bdef-4b52-8923-940bf66ffd8f}"),
            T::type_uuid(),
        )
    }
}

impl<T: TypeUuidProvider> TypeUuidProvider for Vec<T> {
    fn type_uuid() -> UUID {
        combine_uuids(
            UUID::create_string("{144ecd3c-6b34-4db3-91a4-0158f293fe61}"),
            T::type_uuid(),
        )
    }
}

#[inline]
pub fn combine_uuids(a: UUID, b: UUID) -> UUID {
    return a + b;
}

/// VObject provider provides dynamic access to inner vobjects of an object by their type id.
pub trait VObjectProvider {
    /// Allows an object to provide access to inner vobjects.
    fn query_vobject_ref(&self, type_id: TypeId) -> Option<&dyn Any>;

    /// Allows an object to provide access to inner vobjects.
    fn query_vobject_mut(&mut self, type_id: TypeId) -> Option<&mut dyn Any>;
}

impl dyn VObjectProvider {
    /// Tries to borrow a vobject of given type.
    #[inline]
    pub fn vobject_ref<T: Any>(&self) -> Option<&T> {
        VObjectProvider::query_vobject_ref(self, TypeId::of::<T>())
            .and_then(|c| c.downcast_ref())
    }

    /// Tries to borrow a vobject of given type.
    #[inline]
    pub fn vobject_mut<T: Any>(&mut self) -> Option<&mut T> {
        VObjectProvider::query_vobject_mut(self, TypeId::of::<T>())
            .and_then(|c| c.downcast_mut())
    }
}


/// Implements [`VObjectProvider::query_vobject_ref`] and [`VObjectProvider::query_vobject_mut`] in a much
/// shorter way.
#[macro_export]
macro_rules! impl_vobject_provider {
     ($dest_type:ty) => {
        impl $crate::type_traits::VObjectProvider for $dest_type {
            fn query_vobject_ref(&self, type_id: std::any::TypeId) -> Option<&dyn std::any::Any> {
                if type_id == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }
                None
            }

            fn query_vobject_mut(
                &mut self,
                type_id: std::any::TypeId,
            ) -> Option<&mut dyn std::any::Any> {
                if type_id == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }
                None
            }
        }
    };

    ($dest_type:ty, $($($vobj_field:ident).*: $vobj_type:ty),*) => {
        impl $crate::type_traits::VObjectProvider for $dest_type {
            fn query_vobject_ref(&self, type_id: std::any::TypeId) -> Option<&dyn std::any::Any> {
                if type_id == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }

                $(
                    if type_id == std::any::TypeId::of::<$vvobj_type>() {
                        return Some(&self.$($vobj_field).*)
                    }
                )*

                None
            }

            fn query_vobject_mut(
                &mut self,
                type_id: std::any::TypeId,
            ) -> Option<&mut dyn std::any::Any> {
                if type_id == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }

                $(
                    if type_id == std::any::TypeId::of::<$vobj_type>() {
                        return Some(&mut self.$($vobj_field).*)
                    }
                )*

                None
            }
        }
    };
}