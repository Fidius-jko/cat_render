use std::any::TypeId;

pub enum CatTypeId {
    TypeId(TypeId),
    Me(u128, u128),
}
