use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dish {
    // ID -> sort key。これ単体で検索することはないのでsort keyにする
    // created_at_bento_idで一意にする。created_atでsortするので先にする
    pub id: String,
    // 料理名
    pub name: String,
    // 弁当ID -> partition key
    // 料理の一覧を取得したいなら、一度bentoの一覧をuserで取得してそのidで検索する
    pub bento_id: String,
    // お気に入り
    pub favorite: bool,
    // 作成日時
    pub created_at: DateTime<Utc>,
    // 更新日時
    pub updated_at: DateTime<Utc>,
}

impl Dish {
    pub fn new(self) -> Self {
        Self {
            // TODO bento_id_created_atで一意にする
            id: Utc::now().timestamp_millis().to_string(),
            name: self.name,
            bento_id: self.bento_id,
            favorite: self.favorite,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
