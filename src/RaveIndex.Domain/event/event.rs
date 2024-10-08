use chrono::NaiveDateTime;

pub struct Event {
  pub event_name: String,
  pub event_media: EventMedia,
  pub event_location: Location,
  pub event_status: EventStatus,
  pub partner_id: usize,
  pub event_date: Option<NaiveDateTime>
}

pub struct EventMedia {
  pub url: String
}

pub enum EventStatus {
  FUTURE,
  ONGOING,
  CLOSED
}