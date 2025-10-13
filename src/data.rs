use crate::{
    Error,
    routes::{CreateFishRequest, LoginResponse, UpdateFishRequest},
};
use axum::{
    Extension, RequestPartsExt,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use jiff::Timestamp;
use rusqlite::{
    Connection, Row, ToSql, named_params,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    ops::Deref,
    path::Path,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use tracing::info;

/// User specifies their session ID in this header
pub const SESSION_ID_HEADER: &str = "Shoal-Session-Id";

/// Default fish defined for all users
static FISHES: &[StaticFish] = &[
    StaticFish {
        name: "Nemo",
        species: "Clownfish",
        age: 2,
        weight_kg: 0.1,
    },
    StaticFish {
        name: "Dory",
        species: "Blue Tang",
        age: 5,
        weight_kg: 0.3,
    },
    StaticFish {
        name: "Sam",
        species: "Sockeye Salmon",
        age: 5,
        weight_kg: 5.2,
    },
    StaticFish {
        name: "Barry",
        species: "Great Barracuda",
        age: 11,
        weight_kg: 8.3,
    },
];

/// In-memory database for fish. This uses an Arc so it is safe and cheap to
/// clone.
#[derive(Clone, Debug)]
pub struct Store {
    /// SQLite DB. Mutex needed to allow multiple connections to access the DB
    /// at once. Hopefully load is low enough that this isn't an issue
    connection: Arc<Mutex<Connection>>,
}

impl Store {
    pub fn new() -> crate::Result<Self> {
        info!("Opening database");
        let connection = Connection::open_in_memory()?;
        connection.pragma_update(None, "foreign_keys", "ON")?;

        // Initialize the DB
        connection.execute(
            "CREATE TABLE session (
                id TEXT PRIMARY KEY,
                expires_at TEXT
            )",
            (),
        )?;
        connection.execute(
            "CREATE TABLE fish (
                id INTEGER PRIMARY KEY,
                session_id TEXT,
                name TEXT NOT NULL,
                species TEXT NOT NULL,
                age INTEGER NOT NULL,
                weight_kg REAL NOT NULL,
                FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE
            )",
            (),
        )?;
        // Add default fish
        for fish in FISHES {
            connection.execute(
                "INSERT INTO fish (name, species, age, weight_kg)
                VALUES (:name, :species, :age, :weight_kg)",
                named_params! {
                    ":name": fish.name,
                    ":species": fish.species,
                    ":age": fish.age,
                    ":weight_kg": fish.weight_kg,
                },
            )?;
        }

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// Create a new session with a unique ID
    pub async fn create_session(&self) -> crate::Result<LoginResponse> {
        // Sessions last 1 hour
        let expires_at =
            (Timestamp::now() + Duration::from_secs(60 * 60)).to_string();
        let conn = self.connection.lock().await;
        let id = conn
            // Generate an ID in the DB and return it
            .prepare(
                "INSERT INTO session (id, expires_at)
                VALUES (lower(hex(randomblob(16))), :expires_at) RETURNING id",
            )?
            .query_one(named_params! { ":expires_at": &expires_at }, |row| {
                row.get("id")
            })?;

        // Copy all default fish to the new session so they can be modified
        conn.execute(
            "INSERT INTO fish (session_id, name, species, age, weight_kg)
            SELECT :session_id, name, species, age, weight_kg FROM fish
                WHERE session_id is NULL",
            named_params! { ":session_id": &id },
        )?;

        Ok(LoginResponse { id, expires_at })
    }

    /// Delete all expired sessions, returning their IDs
    pub async fn reap_sessions(&self) -> crate::Result<Vec<SessionId>> {
        let conn = self.connection.lock().await;
        let deleted: Vec<SessionId> = conn
            .prepare(
                "DELETE FROM session WHERE expires_at < :now RETURNING id",
            )?
            .query_map(
                named_params! { ":now": Timestamp::now().to_string() },
                |row| row.get::<_, SessionId>("id"),
            )?
            .collect::<Result<_, _>>()?;
        Ok(deleted)
    }

    /// Dump the database to a file
    pub async fn dump(&self, path: &Path) -> crate::Result<()> {
        let conn = self.connection.lock().await;
        conn.backup("main", path, None)?;
        Ok(())
    }

    /// Is the session in the store and unexpired?
    async fn contains_session(
        &self,
        session_id: &SessionId,
    ) -> crate::Result<bool> {
        let conn = self.connection.lock().await;
        let contains = conn
            .prepare(
                "SELECT EXISTS (SELECT id FROM session WHERE id = :id
                AND expires_at > :now)",
            )?
            .query_one(
                named_params! {
                    ":id": session_id,
                    ":now": Timestamp::now().to_string(),
                },
                |row| row.get::<_, bool>(0),
            )?;
        Ok(contains)
    }
}

/// A [Store] filtered to a single session's fish. This can be automatically
/// extracted from a request by pulling the session ID from the
/// `Shoal-Session-ID` header.
///
/// A session is an isolated view of the database. Each user's session is unique
/// and will not affect other sessions.
pub struct SessionStore {
    store: Store,
    /// Session to show/modify fish for. If `None`, use the default fish and
    /// mutations will not be allowed
    session_id: Option<SessionId>,
}

impl SessionStore {
    /// List all fish for this session
    pub async fn list(&self) -> crate::Result<Vec<Fish>> {
        let conn = self.connection().await;
        let fishes = conn
            .prepare(
                // NULL = NULL doesn't work so we need a special clause
                "SELECT * FROM fish WHERE
                session_id IS NULL AND :session_id IS NULL
                OR session_id = :session_id",
            )?
            .query_map::<Fish, _, _>(
                named_params! { ":session_id": self.session_id },
                |row| row.try_into(),
            )?
            .collect::<std::result::Result<_, _>>()?;
        Ok(fishes)
    }

    /// Get a fish by ID for this session. Return `None` if not found
    pub async fn get(&self, id: FishId) -> crate::Result<Fish> {
        let conn = self.connection().await;
        let fish = conn.query_one(
            "SELECT * FROM fish WHERE
                (session_id IS NULL AND :session_id IS NULL
                OR session_id = :session_id)
                AND id = :id",
            named_params! { ":session_id": self.session_id, ":id": id },
            |row| row.try_into(),
        )?;
        Ok(fish)
    }

    /// Create a fish for this session
    pub async fn create(&self, body: CreateFishRequest) -> crate::Result<Fish> {
        let conn = self.connection().await;
        let fish = conn.query_one(
            "INSERT INTO fish (session_id, name, species, age, weight_kg)
            VALUES (:session_id, :name, :species, :age, :weight_kg)
            RETURNING *",
            named_params! {
                ":session_id": self.session_id,
                ":name": body.name,
                ":species": body.species,
                ":age": body.age,
                ":weight_kg": body.weight_kg,
            },
            |row| row.try_into(),
        )?;
        Ok(fish)
    }

    /// Modify a fish by ID for this session. Return the modified fish or `None`
    /// if not found
    pub async fn update(
        &self,
        id: FishId,
        body: UpdateFishRequest,
    ) -> crate::Result<Fish> {
        let conn = self.connection().await;
        let fish = conn.query_one(
            // If any given field is None, we'll update it to its existing
            // value. This only works for non-nullable columns
            "UPDATE fish SET
                name = coalesce(:name, name),
                species = coalesce(:species, species),
                age = coalesce(:age, age),
                weight_kg = coalesce(:weight_kg, weight_kg)
            WHERE session_id = :session_id AND id = :id RETURNING *",
            named_params! {
                ":session_id": self.session_id,
                ":id": id,
                ":name": body.name,
                ":species": body.species,
                ":age": body.age,
                ":weight_kg": body.weight_kg,
            },
            |row| row.try_into(),
        )?;
        Ok(fish)
    }

    /// Delete a fish by ID for this session. Return the deleted fish or `None`
    /// if not found
    pub async fn delete(&self, id: FishId) -> crate::Result<Fish> {
        let conn = self.connection().await;
        let fish = conn.query_one(
            "DELETE FROM fish WHERE session_id = :session_id AND id = :id
            RETURNING *",
            named_params! { ":session_id": self.session_id, ":id": id },
            |row| row.try_into(),
        )?;
        Ok(fish)
    }

    async fn connection(&self) -> impl Deref<Target = Connection> {
        self.store.connection.lock().await
    }
}

impl<S: Send + Sync> FromRequestParts<S> for SessionStore {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        async fn get_session_id(
            parts: &Parts,
            store: &Store,
        ) -> Result<Option<SessionId>, Error> {
            // Pull the session ID from the auth header. If the header isn't
            // present, it's just an unauthenticated request.
            let Some(session_id) = &parts.headers.get(SESSION_ID_HEADER) else {
                return Ok(None);
            };
            // If the session ID isn't valid UTF-8, it's definitely not in the
            // DB so give a Not Found error
            let session_id = SessionId(
                session_id
                    .to_str()
                    .map_err(|_| Error::SessionNotFound {
                        session_id: session_id.as_bytes().to_owned(),
                    })?
                    .to_owned(),
            );

            // Verify the session is in the store
            if store.contains_session(&session_id).await? {
                Ok(Some(session_id))
            } else {
                Err(Error::SessionNotFound {
                    session_id: session_id.0.into_bytes(),
                })
            }
        }

        let Extension(store) = parts
            .extract::<Extension<Store>>()
            .await
            .map_err(IntoResponse::into_response)?;
        let session_id = get_session_id(parts, &store)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self { store, session_id })
    }
}

/// Unique ID for a user session, generated by `POST /login`
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(String);

impl ToSql for SessionId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl FromSql for SessionId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let id = String::column_result(value)?;
        Ok(SessionId(id))
    }
}

/// Unique ID for a fish
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FishId(pub u32);

impl Display for FishId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSql for FishId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl FromSql for FishId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let id = u32::column_result(value)?;
        Ok(FishId(id))
    }
}

/// Just keep swimming swimming swimming...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fish {
    pub id: FishId,
    pub name: String,
    pub species: String,
    pub age: u32,
    pub weight_kg: f64,
}

/// Convert from `SELECT * FROM fish`
impl<'a, 'b> TryFrom<&'a Row<'b>> for Fish {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row<'b>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            species: row.get("species")?,
            age: row.get("age")?,
            weight_kg: row.get("weight_kg")?,
        })
    }
}

/// Fish defined in static code
struct StaticFish {
    name: &'static str,
    species: &'static str,
    age: u32,
    weight_kg: f64,
}
