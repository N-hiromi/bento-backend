use common::models::bento::Bento;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BentoListResponse {
    pub id: String,
    pub favorite: bool,
    pub image_path: String,
}

impl From<Bento> for BentoListResponse {
    fn from(bento: Bento) -> BentoListResponse {
        BentoListResponse {
            id: bento.id,
            favorite: bento.favorite,
            image_path: bento.image_path,
        }
    }
}
