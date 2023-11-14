use std::sync::Arc;

use axum::{extract::{State, Path}, Json, response::IntoResponse};
use sea_orm::DatabaseConnection;

use crate::services::{
    create_customer_service::{CreateCustomertDto, CreateCustomerServiceBuilder, CreateCustomertDtoBuilder}, 
    show_customer_service::{ShowCustomerPath, ShowCustomerPathBuilder, ShowCustomerServiceBuilder}, list_customers_service::ListCustomersServiceBuilder, delete_customer_service::{DeleteCustomerPath, DeleteCustomerServiceBuilder, DeleteCustomerPathBuilder}, update_customer_service::{UpdateCustomerPath, UpdateCustomerDto, UpdateCustomerServiceBuilder, UpdateCustomerPathBuilder, UpdateCustomerDtoBuilder}
};

pub struct CustomersController {}
impl CustomersController 
{
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(create_customers_dto): Json<CreateCustomertDto>) -> impl IntoResponse
    {
        let create_customers_service = CreateCustomerServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let create_customers_dto = CreateCustomertDtoBuilder::default()
        .name(create_customers_dto.name)
        .email(create_customers_dto.email)
        .build()
        .unwrap();

        let customer = create_customers_service.execute(create_customers_dto).await;

        match customer 
        {
            Ok(_) =>
            {
                return "Customer Criado.".into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }   
    pub async fn show(State(connection): State<Arc<DatabaseConnection>>, Path(show_customer_path): Path<ShowCustomerPath>) -> impl IntoResponse
    {
        let show_customer_service = ShowCustomerServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let show_customer_path = ShowCustomerPathBuilder::default()
        .id(show_customer_path.id)
        .build()
        .unwrap();

        let customer = show_customer_service.execute(show_customer_path).await;

        match customer 
        {
            Ok(c) =>
            {
                let customer = serde_json::to_string(&c).unwrap();
                return customer.into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }
    pub async fn list(State(connection): State<Arc<DatabaseConnection>>) -> impl IntoResponse 
    {
        let list_customer_service = ListCustomersServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let customers = list_customer_service.execute().await.unwrap();

        let customers = serde_json::to_string(&customers).unwrap();

        customers
    }
    pub async fn delete(State(connection): State<Arc<DatabaseConnection>>, Path(delete_customer_path): Path<DeleteCustomerPath>) -> impl IntoResponse
    {
        let delete_customer_service = DeleteCustomerServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let delete_customer_path = DeleteCustomerPathBuilder::default()
        .id(delete_customer_path.id)
        .build()
        .unwrap();

        let customer = delete_customer_service.execute(delete_customer_path).await;

        match customer 
        {
            Ok(_) =>
            {
                return "[]".into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }
    pub async fn update(State(connection): State<Arc<DatabaseConnection>>, Path(update_customer_path): Path<UpdateCustomerPath>, Json(update_customer_dto): Json<UpdateCustomerDto>) -> impl IntoResponse
    {
        let update_customer_service = UpdateCustomerServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let update_customer_path = UpdateCustomerPathBuilder::default()
        .id(update_customer_path.id)
        .build()
        .unwrap();

        let update_customer_dto = UpdateCustomerDtoBuilder::default()
        .name(update_customer_dto.name)
        .email(update_customer_dto.email)
        .build()
        .unwrap();

        let customer = update_customer_service.execute(update_customer_path, update_customer_dto).await;

        match customer 
        {
            Ok(p) =>
            {
                let customer = serde_json::to_string(&p).unwrap();
                return customer.into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }
}
