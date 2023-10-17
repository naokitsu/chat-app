use super::{DatabaseError, DatabaseManager, Hash, Id, Message, Room, User};
use sqlx::error::Error as SQLXError;
use sqlx::{Pool, Sqlite, SqlitePool};
struct SQLite {
    pool: Pool<Sqlite>,
}

impl SQLite {
    async fn new(url: &str) -> Result<Self, SQLXError> {
        Ok(SQLite {
            pool: SqlitePool::connect(url).await?,
        })
    }
}

#[async_trait]
impl DatabaseManager for SQLite {
    async fn setup(&self) -> Result<(), DatabaseError> {
        let mut connection = self
            .pool
            .acquire()
            .await
            .map_err(|x| DatabaseError::ConnectionFailed(x))?;

        let query_result = sqlx::query!(
            r#"
CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL CHECK(length(username) >= 2 AND length(username) <= 32),
    hash BLOB(48) NOT NULL
);

CREATE TABLE IF NOT EXISTS Rooms (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL CHECK(length(name) >= 2 AND length(name) <= 32),
    creator INTEGER NOT NULL,
    FOREIGN KEY (creator) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Messages (
    id INTEGER PRIMARY KEY,
    sender INTEGER NOT NULL,
    room INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (sender) REFERENCES Users(id),
    FOREIGN KEY (room) REFERENCES Rooms(id)
);

CREATE TABLE IF NOT EXISTS Relations (
    user INTEGER NOT NULL,
    room INTEGER NOT NULL,
    FOREIGN KEY (user) REFERENCES Users(id),
    FOREIGN KEY (room) REFERENCES Rooms(id),
    PRIMARY KEY (user, room)
);

CREATE TABLE IF NOT EXISTS Sessions (
    id INTEGER PRIMARY KEY,
    user INTEGER NOT NULL,
    token TEXT NOT NULL,
    expire TIMESTAMP,
    description TEXT,
    FOREIGN KEY (user) REFERENCES Users(id)
);
"#
        )
        .fetch_one(&mut *connection)
        .await
        .map_err(|x| DatabaseError::QueryFailed(x))?;
    }

    async fn teardown(&self) -> Result<(), DatabaseError> {
        let mut connection = self
            .pool
            .acquire()
            .await
            .map_err(|x| DatabaseError::ConnectionFailed(x))?;

        let query_result = sqlx::query!(
            r#"
DROP TABLE IF EXISTS Sessions;
DROP TABLE IF EXISTS Relations;
DROP TABLE IF EXISTS Messages;
DROP TABLE IF EXISTS Rooms;
DROP TABLE IF EXISTS Users;
"#
        )
        .fetch_one(&mut *connection)
        .await
        .map_err(|x| DatabaseError::QueryFailed(x))?;
    }

    fn add_user(&self, user: User, hash: Hash) -> Result<(), ()> {
        todo!()
    }

    fn remove_user(&self, id: Id) -> Result<(), ()> {
        todo!()
    }

    fn select_user(&self, id: Id) -> Option<User> {
        todo!()
    }

    fn add_room(&self, room: Room) -> Result<(), ()> {
        todo!()
    }

    fn remove_room(&self, id: Id) -> Result<(), ()> {
        todo!()
    }

    fn select_room(&self, id: Id) -> Option<Room> {
        todo!()
    }

    fn add_message(&self, message: Message) -> Result<(), ()> {
        todo!()
    }

    fn remove_message(&self, id: Id) -> Result<(), ()> {
        todo!()
    }

    fn select_message(&self, id: Id) -> Option<Message> {
        todo!()
    }
}
