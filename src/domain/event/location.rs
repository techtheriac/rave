pub struct Location {
  pub location_address: String,
  pub location_status: LocationStatus
}

pub enum LocationStatus {
  TO_BE_ANNOUNCED,
  ANNOUNCED
}