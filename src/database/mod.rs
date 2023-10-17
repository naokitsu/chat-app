mod sqlite;

use sqlx::Error;
use std::time::SystemTime;

enum DatabaseError {
    ConnectionFailed(Error),
    QueryFailed(Error),
}

#[async_trait]
trait DatabaseManager {
    async fn setup(&self) -> Result<(), DatabaseError>;

    async fn teardown(&self) -> Result<(), DatabaseError>;

    async fn add_user(&self, user: User, hash: Hash) -> Result<(), ()>;

    async fn remove_user(&self, id: Id) -> Result<(), ()>;

    async fn select_user(&self, id: Id) -> Option<User>;

    async fn add_room(&self, room: Room) -> Result<(), ()>;

    async fn remove_room(&self, id: Id) -> Result<(), ()>;

    async fn select_room(&self, id: Id) -> Option<Room>;

    async fn add_message(&self, message: Message) -> Result<(), ()>;

    async fn remove_message(&self, id: Id) -> Result<(), ()>;

    async fn select_message(&self, id: Id) -> Option<Message>;
}

type Id = u64;
type Hash = Vec<u8>;

struct User {
    id: Id,
    name: String,
}

struct Room {
    id: Id,
    name: String,
    creator: Id,
}

struct Message {
    id: Id,
    sender: Id,
    room: Id,
    content: String,
}
struct Session {
    id: Id,
    user: Id,
    token: String,
    expire: SystemTime,
    description: String,
}
