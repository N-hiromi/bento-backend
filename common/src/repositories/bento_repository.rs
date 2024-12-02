use crate::models::bento::Bento;
use crate::repositories::repository::{Repository, RepositoryError};
use crate::utils::time::get_month_range;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use serde_dynamo::from_items;
use std::collections::HashMap;
use std::future::Future;

static TABLE_NAME: &str = "bento";

pub trait BentoRepository {
    fn query(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<Vec<Bento>, RepositoryError>> + Send;
    fn get_month(
        &self,
        user_id: &str,
        target_year: i32,
        target_month: u32,
    ) -> impl Future<Output = Result<Vec<Bento>, RepositoryError>> + Send;
    fn get_id(&self, id: &str) -> impl Future<Output = Result<Bento, RepositoryError>> + Send;
    fn get_favorites(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<Vec<Bento>, RepositoryError>> + Send;
    fn post(&self, bento: Bento) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn put(&self, bento: Bento) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn delete(&self, id: &str) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}

impl BentoRepository for Repository {
    // 全件取得する
    async fn query(&self, user_id: &str) -> Result<Vec<Bento>, RepositoryError> {
        let result = self
            .client
            .query()
            .table_name(self.env_table_name(TABLE_NAME))
            .key_condition_expression("#user_id = :user_id")
            .expression_attribute_names("#user_id", "user_id")
            .expression_attribute_values(":user_id", AttributeValue::S(user_id.to_string()))
            // sort key(bento_id: エポックミリ秒)で降順
            .scan_index_forward(false)
            .send()
            .await?;
        let items = from_items(result.items().to_vec())?;
        Ok(items)
    }

    // 月を指定して取得する
    async fn get_month(
        &self,
        user_id: &str,
        target_year: i32,
        target_month: u32,
    ) -> Result<Vec<Bento>, RepositoryError> {
        let (start_time, end_time) = get_month_range(target_year, target_month);
        // 一旦全件取得する
        let all_items = BentoRepository::query(self, user_id).await?;
        // 月を絞り込む
        let items: Vec<Bento> = all_items
            .into_iter()
            .filter(|item| {
                item.get_time().unwrap() >= start_time || item.get_time().unwrap() < end_time
            })
            .collect();
        Ok(items)
    }

    // idを指定して取得する
    async fn get_id(&self, id: &str) -> Result<Bento, RepositoryError> {
        let result = self
            .client
            .query()
            .table_name(self.env_table_name(TABLE_NAME))
            .key_condition_expression("#id = :id")
            .expression_attribute_names("#id", "id")
            .expression_attribute_values(":id", AttributeValue::S(id.to_string()))
            // sort key(bento_id: エポックミリ秒)で降順
            .send()
            .await?;
        let items: Vec<Bento> = from_items(result.items().to_vec())?;
        // 1件取得する
        match items.first() {
            Some(item) => Ok(item.clone()),
            // 空の場合不正なidなのでエラーを返す
            None => Err(RepositoryError::NotFound),
        }
    }

    // お気に入り一覧を返す
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<Bento>, RepositoryError> {
        // 一旦全件取得する
        let all_items = BentoRepository::query(self, user_id).await?;
        // お気に入りを絞り込む
        let items: Vec<Bento> = all_items
            .into_iter()
            .filter(|item| item.favorite == true)
            .collect();
        Ok(items)
    }

    // 登録
    async fn post(&self, bento: Bento) -> Result<(), RepositoryError> {
        self.client
            .put_item()
            .table_name(self.env_table_name(TABLE_NAME))
            .set_item(Some(to_item(bento)?))
            .send()
            .await?;
        Ok(())
    }

    // 編集
    async fn put(&self, bento: Bento) -> Result<(), RepositoryError> {
        // 更新する値
        let favorite_av = AttributeValue::Bool(bento.favorite);
        let image_path_av = AttributeValue::S(bento.image_path);

        // 更新する値(Optionalな値)
        let memo_av = AttributeValue::S(bento.memo.clone().unwrap_or("".to_string()));
        let updated_at_av = match bento.updated_at {
            Some(updated_at) => AttributeValue::N(updated_at.to_rfc3339()),
            None => AttributeValue::N("".to_string()),
        };

        // 更新処理式
        let mut update_expression =
            "SET #favorite = :favorite, #image_path = :image_path".to_string();
        let mut expression_attribute_values: HashMap<String, AttributeValue> = HashMap::from([
            (":favorite".to_string(), favorite_av),
            (":image_path".to_string(), image_path_av),
        ]);

        // Optionalな値がある場合は更新処理式に追加
        if bento.memo.is_some() {
            update_expression.push_str(", #memo = :memo");
            expression_attribute_values.insert(":memo".to_string(), memo_av);
        }

        if bento.updated_at.is_some() {
            update_expression.push_str(", #updated_at = :updated_at");
            expression_attribute_values.insert(":updated_at".to_string(), updated_at_av);
        }
        // ------------------------------

        // 更新処理
        self.client
            .update_item()
            .table_name(self.env_table_name(TABLE_NAME))
            .key("id", AttributeValue::S(bento.id.to_string()))
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
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
