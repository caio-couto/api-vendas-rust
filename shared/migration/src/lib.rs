pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_products;
mod m20231104_135233_create_users;
mod m20231111_234218_user_tokens;
mod m20231114_055850_create_customers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator 
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>> 
    {
        vec![
            Box::new(m20220101_000001_create_products::Migration),
            Box::new(m20231104_135233_create_users::Migration),
            Box::new(m20231111_234218_user_tokens::Migration),
            Box::new(m20231114_055850_create_customers::Migration),
        ]
    }
}
