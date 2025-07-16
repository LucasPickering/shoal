use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::ToSchema;

pub static FISHES: LazyLock<IndexMap<FishId, Fish>> = LazyLock::new(|| {
    [
        Fish {
            id: FishId(1),
            name: "Nemo".to_string(),
            species: "Clownfish".to_string(),
            age: 2,
            weight_kg: 0.1,
        },
        Fish {
            id: FishId(2),
            name: "Dory".to_string(),
            species: "Blue Tang".to_string(),
            age: 5,
            weight_kg: 0.3,
        },
        Fish {
            id: FishId(3),
            name: "Sam".to_string(),
            species: "Sockeye Salmon".to_string(),
            age: 5,
            weight_kg: 5.2,
        },
        Fish {
            id: FishId(4),
            name: "Barry".to_string(),
            species: "Great Barracuda".to_string(),
            age: 11,
            weight_kg: 8.3,
        },
    ]
    .into_iter()
    .map(|fish| (fish.id, fish))
    .collect()
});

/// TODO
#[derive(
    Copy,
    Clone,
    Debug,
    derive_more::Display,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[serde(transparent)]
pub struct FishId(pub u32);

/// TODO
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Fish {
    pub id: FishId,
    pub name: String,
    pub species: String,
    pub age: u32,
    pub weight_kg: f64,
}
