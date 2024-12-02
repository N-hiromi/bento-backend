use crate::services::bento_get_service::BentoGetData;
use common::models::dish::Dish;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BentoGetResponse {
    pub id: String,
    pub image_path: String,
    pub favorite: bool,
    pub memo: String,
    pub dishes: Vec<Dish>,
}

impl From<BentoGetData> for BentoGetResponse {
    fn from(bento: BentoGetData) -> BentoGetResponse {
        BentoGetResponse {
            id: bento.id,
            image_path: bento.image_path,
            favorite: bento.favorite,
            memo: bento.memo,
            dishes: bento.dishes,
        }
    }
}
