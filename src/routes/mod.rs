pub mod rooms;
pub mod players;
use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "Error",
        "reason": "Resource was not found."
    })
}
