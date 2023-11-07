use std::{sync::Arc, ops::Deref};
use derive_builder::Builder;
use sea_orm::{ DatabaseConnection, ActiveValue::{ NotSet, Set }, DbErr, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use crate::entity::users::{ActiveModel, Entity as Users, self};
use bcrypt::{DEFAULT_COST, hash};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct CreateUserDto 
{
    pub name: String, 
    pub email: String, 
    pub password: String,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateUserService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateUserService 
{
    pub async fn execute(&self, create_user_dto: CreateUserDto) -> Result<ActiveModel, DbErr>
    {
        let password = hash(create_user_dto.password, DEFAULT_COST).unwrap(); 
            
        let new_user = ActiveModel
        {
            id: NotSet,
            name: Set(create_user_dto.name),
            email: Set(create_user_dto.email),
            password: Set(password),
            avatar: NotSet,
            created_at: NotSet,
            updated_at: NotSet
        };
        
        let new_user = new_user.save(self.connection.deref()).await?;
        
        Ok(new_user)
    }    
}
