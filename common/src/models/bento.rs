use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bento {
    // 弁当ID(userの指定日のエポックタイムミリ秒_sub) -> sort key
    // 一意なId。前方一致でsortできるように指定日を先頭にする
    pub id: String,
    // ユーザID -> partition key
    pub user_id: String,
    // お気に入り
    pub favorite: bool,
    // 弁当画像のS3パス
    pub image_path: String,
    // メモ
    pub memo: Option<String>,
    // created_at
    pub created_at: DateTime<Utc>,
    // updated_at
    pub updated_at: Option<DateTime<Utc>>,
}

impl Bento {
    // 新規登録
    pub fn new(
        user_id: String,
        target_date: i64,
        favorite: bool,
        image_path: String,
        memo: Option<String>,
    ) -> Self {
        Self {
            id: target_date.to_string() + "_" + user_id.as_str(),
            user_id,
            favorite,
            image_path,
            memo,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    // 更新
    pub fn update(&mut self, favorite: bool, image_path: String, memo: Option<String>) {
        self.favorite = favorite;
        self.image_path = image_path;
        self.memo = memo;
        self.updated_at = Some(Utc::now());
    }

    // bento_idはtarget_time_user_idの形式なので、target_timeを取得する
    pub fn get_time(&self) -> Result<i64, BentoIdError> {
        self.id
            .split('_')
            .next()
            .ok_or(BentoIdError::TargetTimeNotFound)?
            .parse()
            .map_err(BentoIdError::ParseIntError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BentoIdError {
    #[error("Invalid Bento ID. Target_time is not found.")]
    TargetTimeNotFound,
    #[error("invalid Bento ID. Target_time parse as i64 is failed.")]
    ParseIntError(#[from] std::num::ParseIntError),
}
