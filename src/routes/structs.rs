use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Fish {
    pub fish_id: Uuid,
    pub fish_type_id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f32>,
    pub omega_3: Option<f32>,
    pub omega_3_ratio: Option<f32>,
    pub pcb: Option<f32>,
    pub protein: Option<f32>,
    pub lake: String,
    pub about: String,
}

#[derive(serde::Serialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub ingredients: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FishType {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) anishinaabe_name: Option<String>,
    pub(crate) fish_image: Option<String>,
    pub(crate) s3_fish_image: Option<String>,
    pub(crate) s3_woodland_image: Option<String>,
    pub(crate) woodland_fish_image: Option<String>,
    pub(crate) about: String,
}
