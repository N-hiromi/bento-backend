use common::models::bento::Bento;
use common::models::dish::Dish;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BentoRequest {
    pub target_date: i64,    // 日付
    pub favorite: bool,      // お気に入り
    pub image_path: String,  // 画像パス
    pub memo: String,        // メモ
    pub dishes: Vec<String>, // メニュー
}

impl BentoRequest {
    pub fn into_bento(self, user_id: String) -> Bento {
        Bento::new(
            user_id,
            self.target_date,
            self.favorite,
            self.image_path,
            Some(self.memo),
        )
    }

    pub fn into_bento_update(self, bento: &mut Bento) {
        bento.update(self.favorite, self.image_path, Some(self.memo));
    }

    pub fn into_dish(self) -> Vec<Dish> {
        self.dishes
            .iter()
            .map(|dish| Dish::new(dish.clone()))
            .collect()
    }
}
