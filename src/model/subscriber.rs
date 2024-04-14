use rocket::serde::{Serialize, Deserialize};
#[derive(Debug, Clone. Serealize, Deserealize)]
#[serde(crate="rocket::serde")]
pub struct SubscriberRequest{
    pub url: String,
    pub name: String
}
