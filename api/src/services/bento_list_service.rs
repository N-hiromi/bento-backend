use common::models::bento::Bento;
use common::repositories::bento_repository::BentoRepository;
use common::repositories::repository::RepositoryError;

pub async fn execute(
    bento_repository: &impl BentoRepository,
    user_id: String,
) -> Result<Vec<Bento>, RepositoryError> {
    let bentos = bento_repository.query(&user_id).await?;
    Ok(bentos)
}
