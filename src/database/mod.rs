use std::time::SystemTime;

trait DatabaseManager {
    fn setup(&self);

    fn teardown(&self);

    fn add_user(&self, user: User, hash: Hash) -> Result<(), ()>;

    fn remove_user(&self, id: Id) -> Result<(), ()>;

    fn select_user(&self, id: Id) -> Option<User>;

    fn add_room(&self, room: Room) -> Result<(), ()>;

    fn remove_room(&self, id: Id) -> Result<(), ()>;

    fn select_room(&self, id: Id) -> Option<Room>;
    
    fn add_message(&self, message: Message) -> Result<(), ()>;
    
    fn remove_message(&self, id: Id) -> Result<(), ()>;
    
    fn select_message(&self, id: Id) -> Option<Message>;
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
