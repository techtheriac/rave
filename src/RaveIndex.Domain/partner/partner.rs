pub struct Partner {
  pub partner_id: usize,
  pub alias: String,
  pub organizes: vec<Event>,
  pub partner_status: PartnerStatus
}

pub enum PartnerStatus {
  PENDING_VERIFICATION,
  VERIFIED
}