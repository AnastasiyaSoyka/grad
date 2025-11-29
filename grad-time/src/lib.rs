//! Time and date library logic.

const FMT: &str = "%F %T";

/// UTC
pub mod utc {
    use chrono::Utc;

    pub fn now() -> Vec<u8> {
        let now = Utc::now();

        now.format(super::FMT).to_string().into_bytes()
    }

    pub fn now_iso8601() -> Vec<u8> {
        let now = Utc::now();

        now.to_rfc3339().into_bytes()
    }

    pub fn now_rfc3339() -> Vec<u8> {
        let now = Utc::now();

        now.to_rfc3339().into_bytes()
    }

    pub fn now_rfc2822() -> Vec<u8> {
        let now = Utc::now();

        now.to_rfc2822().into_bytes()
    }
}

/// Localized Timezone
pub mod local {
    use chrono::Local;

    pub fn now() -> Vec<u8> {
        let now = Local::now();

        now.format(super::FMT).to_string().into_bytes()
    }

    pub fn now_iso8601() -> Vec<u8> {
        let now = Local::now();

        now.to_rfc3339().into_bytes()
    }

    pub fn now_rfc3339() -> Vec<u8> {
        let now = Local::now();

        now.to_rfc3339().into_bytes()
    }

    pub fn now_rfc2822() -> Vec<u8> {
        let now = Local::now();

        now.to_rfc2822().into_bytes()
    }
}
