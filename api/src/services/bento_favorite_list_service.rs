use crate::controllers::controller::ApiRepositoryError;
use common::models::bento::Bento;
use common::repositories::bento_repository::BentoRepository;

pub async fn execute(
    repository: impl BentoRepository,
    user_id: &str,
) -> Result<Vec<Bento>, ApiRepositoryError> {
    repository
        .get_favorites(user_id)
        .await
        .map_err(|e| ApiRepositoryError { e })
}
