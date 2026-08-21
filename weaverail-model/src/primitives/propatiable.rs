use crate::path::Heddle;

/// 拡張プロパティを保持する構造体を表すトレイト
pub trait PropertiableObject {
    /// 拡張プロパティの値を取得する
    fn get_property(&self, id: &str) -> Option<&Heddle>;
    /// 拡張プロパティの値を設定する
    fn set_property(&mut self, id: &str, value: Heddle) -> Option<Heddle>;
    /// 拡張プロパティの値を削除する
    fn remove_property(&mut self, id: &str) -> Option<Heddle>;
}
