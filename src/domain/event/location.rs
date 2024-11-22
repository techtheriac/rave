pub struct Location {
  pub location_address: String,
  pub location_status: LocationStatus,
  pub coord: String
}

pub enum LocationStatus {
  TO_BE_ANNOUNCED,
  ANNOUNCED
}
