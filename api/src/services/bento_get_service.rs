use crate::controllers::controller::ApiRepositoryError;
use common::models::bento::Bento;
use common::models::dish::Dish;
use common::repositories::bento_repository::BentoRepository;
use common::repositories::dish_repository::DishRepository;
use common::repositories::repository::Repository;

pub async fn execute(
    repository: Repository,
    bento_id: &str,
) -> Result<BentoGetData, ApiRepositoryError> {
    let bento = repository
        .get_id(bento_id)
        .await
        .map_err(|e| ApiRepositoryError { e })?;
    Ok(from_bento(bento, repository).await?)
}

pub struct BentoGetData {
    pub id: String,
    pub favorite: bool,
    pub image_path: String,
    pub memo: String,
    pub dishes: Vec<Dish>,
}

// fromトレイトは引数を1つしか使えなかったのと型変換の中に検索処理を入れるのは違う気がした
async fn from_bento(
    bento: Bento,
    repository: impl DishRepository,
) -> Result<BentoGetData, ApiRepositoryError> {
    let dishes = repository
        .query_bento_id(&bento.id)
        .await
        .map_err(|e| ApiRepositoryError { e })?;

    Ok(BentoGetData {
        id: bento.id,
        favorite: bento.favorite,
        image_path: bento.image_path,
        memo: bento.memo,
        dishes,
    })
}
