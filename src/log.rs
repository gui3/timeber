use chrono::{Utc, DateTime};

pub struct Log {
	pub date_created: DateTime<Utc>,
	pub date_updated: DateTime<Utc>,
	pub brief: String
}

pub fn new_log(brief: String) -> Log {
	return Log {
		date_created: Utc::now(),
		date_updated: Utc::now(),
		brief,
	};
}

pub fn update_log(log: &mut Log, brief: String) {
	log.brief = brief;
	log.date_updated = Utc::now();
}
