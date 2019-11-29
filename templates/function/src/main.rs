use serde_json::json;
use actix_web::error;
use cloudevent::{Event, Reader, Writer};
use faas_rust_macro::faas_function;

#[faas_function]
pub async fn function(input: Option<Event>) -> Result<Option<Event>, actix_web::Error> {
    let event = input
        .ok_or(error::ErrorBadRequest("Missing event"))?;
    let json_payload = event.read_payload()
        .ok_or(error::ErrorBadRequest("Event is missing a payload"))??;
    let name = json_payload
        .as_object()
        .and_then(|o| o.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("World");
    let json = json!({
        "Hello": name,
    });
    let mut output: Event = Event::new();
    output.write_payload("application/json", json)?;
    Ok(Some(output))
}