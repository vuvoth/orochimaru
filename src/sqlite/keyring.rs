//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "keyring")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub public_key: String,
    pub secret_key: String,
    pub created_date: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::randomness::Entity")]
    Randomness,
}

impl Related<super::randomness::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Randomness.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
