use std::sync::Arc;

use axum::{extract::{State, Path}, Json, response::IntoResponse};
use sea_orm::DatabaseConnection;

use crate::services::{
    create_products_service::{CreateProductServiceBuilder, CreateProductDtoBuilder, CreateProductDto},
    show_product_service::{ShowProductServiceBuilder, ShowProductPath, ShowProductPathBuilder}, 
    list_product_service::ListProductServiceBuilder, 
    delete_product_service::{DeleteProductPath, DeleteProductServiceBuilder, DeleteProductPathBuilder}, 
    update_product_service::{UpdateProductDto, UpdateProductServiceBuilder, UpdateProductDtoBuilder, UpdateProductPathBuilder, UpdateProductPath}};

pub struct ProductsController {}
impl ProductsController 
{
    pub async fn create(State(connection): State<Arc<DatabaseConnection>>, Json(products_dto): Json<CreateProductDto>) -> impl IntoResponse
    {
        let create_products_service = CreateProductServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let create_product_dto = CreateProductDtoBuilder::default()
        .name(products_dto.name)
        .price(products_dto.price)
        .quantity(products_dto.quantity)
        .build()
        .unwrap();

        let product = create_products_service.execute(create_product_dto).await;

        match product 
        {
            Ok(_) =>
            {
                return "Produto Criado.".into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }   
    pub async fn show(State(connection): State<Arc<DatabaseConnection>>, Path(show_product_path): Path<ShowProductPath>) -> impl IntoResponse
    {
        let show_product_service = ShowProductServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let show_product_path = ShowProductPathBuilder::default()
        .id(show_product_path.id)
        .build()
        .unwrap();

        let product = show_product_service.execute(show_product_path).await;

        match product 
        {
            Ok(p) =>
            {
                let product = serde_json::to_string(&p).unwrap();
                return product.into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }
    pub async fn list(State(connection): State<Arc<DatabaseConnection>>) -> impl IntoResponse 
    {
        let list_product_service = ListProductServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let products = list_product_service.execute().await.unwrap();

        let products = serde_json::to_string(&products).unwrap();

        products
    }
    pub async fn delete(State(connection): State<Arc<DatabaseConnection>>, Path(delete_product_path): Path<DeleteProductPath>) -> impl IntoResponse
    {
        let delete_product_service = DeleteProductServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let delete_product_path = DeleteProductPathBuilder::default()
        .id(delete_product_path.id)
        .build()
        .unwrap();

        let product = delete_product_service.execute(delete_product_path).await;

        match product 
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
    pub async fn update(State(connection): State<Arc<DatabaseConnection>>, Path(update_product_path): Path<UpdateProductPath>, Json(update_product_dto): Json<UpdateProductDto>) -> impl IntoResponse
    {
        let update_product_service = UpdateProductServiceBuilder::default()
        .connection(connection)
        .build()
        .unwrap();

        let update_product_path = UpdateProductPathBuilder::default()
        .id(update_product_path.id)
        .build()
        .unwrap();

        let update_product_dto = UpdateProductDtoBuilder::default()
        .name(update_product_dto.name)
        .price(update_product_dto.price)
        .quantity(update_product_dto.quantity)
        .build()
        .unwrap();

        let product = update_product_service.execute(update_product_path, update_product_dto).await;

        match product 
        {
            Ok(p) =>
            {
                let product = serde_json::to_string(&p).unwrap();
                return product.into_response();
            },
            Err(e) =>
            {
                return e.into_response();
            }
        }
    }
}
