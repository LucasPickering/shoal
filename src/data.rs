use crate::routes::ErrorDetail;
use axum::{Json, http::StatusCode};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    sync::LazyLock,
};
use utoipa::ToSchema;

static FISHES: LazyLock<IndexMap<FishId, Fish>> = LazyLock::new(|| {
    [
        // IDs will be set automatically
        Fish {
            id: FishId(0),
            name: "Nemo".to_string(),
            species: "Clownfish".to_string(),
            age: 2,
            weight_kg: 0.1,
        },
        Fish {
            id: FishId(0),
            name: "Dory".to_string(),
            species: "Blue Tang".to_string(),
            age: 5,
            weight_kg: 0.3,
        },
        Fish {
            id: FishId(0),
            name: "Sam".to_string(),
            species: "Sockeye Salmon".to_string(),
            age: 5,
            weight_kg: 5.2,
        },
        Fish {
            id: FishId(0),
            name: "Barry".to_string(),
            species: "Great Barracuda".to_string(),
            age: 11,
            weight_kg: 8.3,
        },
    ]
    .into_iter()
    .enumerate()
    .map(|(id, mut fish)| {
        fish.id = FishId(id as u32);
        (fish.id, fish)
    })
    .collect()
});

/// Data store for our fishes
pub struct Store;

impl Store {
    /// Get the next valid unused fish ID
    pub fn next_id() -> FishId {
        FishId(
            FISHES
                .keys()
                .map(|id| id.0)
                .max()
                .map(|id| id + 1)
                .unwrap_or(0),
        )
    }

    /// List all fish
    pub fn all() -> impl Iterator<Item = &'static Fish> {
        FISHES.values()
    }

    /// Get a fish by ID or return a 404 response if not found
    pub fn get_or_404(
        id: FishId,
    ) -> Result<&'static Fish, (StatusCode, Json<ErrorDetail>)> {
        FISHES.get(&id).ok_or_else(|| ErrorDetail::not_found(id))
    }
}

/// TODO
#[derive(
    Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, ToSchema,
)]
#[serde(transparent)]
pub struct FishId(pub u32);

impl Display for FishId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// TODO
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Fish {
    pub id: FishId,
    pub name: String,
    pub species: String,
    pub age: u32,
    pub weight_kg: f64,
}
