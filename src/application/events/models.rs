use chrono::NaiveDateTime;
use actix_web::web;

#[derive(Deseriaize, Serialize, Debug, Clone)]
pub struct OrganizeEventRequest {
  pub location: String,
  pub event_name: String,
  pub event_media_url: String,
  pub event_date: Option<NaiveDateTime>
}

impl From<web::Json<OrganizeEventRequest>> for OrganizeEventRequest {
  fn from(request: web::Json<OrganizeEventRequest>) -> Self {
    OrganizeEventRequest {
      location: request.location.clone(),
      event_name: request.event_name.clone(),
      event_media_url: request.event_media_url.clone(),
      event_date: request.event_date
    }
  }  
}
