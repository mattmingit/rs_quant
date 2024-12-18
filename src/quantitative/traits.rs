/// Returns traits
pub trait ReturnDatetime {
    fn datetime(&self) -> &str;
}

pub trait AssetReturn {
    fn asset_return(&self) -> f64;
}
