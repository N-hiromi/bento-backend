use crate::models::dish::Dish;
use crate::repositories::repository::{Repository, RepositoryError};
use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use serde_dynamo::from_items;
use std::future::Future;

static TABLE_NAME: &str = "dish";

pub trait DishRepository {
    fn query_bento_id(
        &self,
        bento_id: &str,
    ) -> impl Future<Output = Result<Vec<Dish>, RepositoryError>> + Send;
    // fn get_favorites(
    //     &self,
    //     user_id: &str,
    // ) -> impl Future<Output = Result<Vec<Dish>, RepositoryError>> + Send;
    fn post(&self, dish: Dish) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}

impl DishRepository for Repository {
    // bentoに紐づくdishを取得する
    async fn query_bento_id(&self, bento_id: &str) -> Result<Vec<Dish>, RepositoryError> {
        let result = self
            .client
            .query()
            .table_name(self.env_table_name(TABLE_NAME))
            .key_condition_expression("#bento_id = :bento_id")
            .expression_attribute_names("#bento_id", "bento_id")
            .expression_attribute_values(":bento_id", AttributeValue::S(bento_id.to_string()))
            // sort key(dish_id: エポックミリ秒)で降順
            .scan_index_forward(false)
            .send()
            .await?;
        let items = from_items(result.items().to_vec())?;
        Ok(items)
    }

    // お気に入り一覧を返す
    // async fn get_favorites(&self, user_id: &str) -> Result<Vec<Dish>, RepositoryError> {
    //     // 一旦全件取得する
    //     let all_items = DishRepository::query(self, user_id).await?;
    //     // お気に入りを絞り込む
    //     let items: Vec<Dish> = all_items
    //         .into_iter()
    //         .filter(|item| item.favorite == true)
    //         .collect();
    //     Ok(items)
    // }

    // 登録
    async fn post(&self, dish: Dish) -> Result<(), RepositoryError> {
        self.client
            .put_item()
            .table_name(self.env_table_name(TABLE_NAME))
            .set_item(Some(to_item(dish)?))
            .send()
            .await?;
        Ok(())
    }

    // 削除
    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        self.client
            .delete_item()
            .table_name(self.env_table_name(TABLE_NAME))
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await?;
        Ok(())
    }
}
