use crate::controllers::controller::ApiRepositoryError;
use common::repositories::repository::Repository;

pub async fn execute(
    repository: &Repository,
    favorite: bool,
    image_path: String,
    memo: String,
    dishes: Vec<String>,
) -> Result<(), ApiRepositoryError> {
    // リクエストの値からbentoを作成する
    let bento =
        // DBにbentoを登録する
        repository
            .create_bento(favorite, image_path, memo, dishes)
            .await
            .map_err(|e| CreateBentoError::SystemError { e })?;
    Ok(())
}
