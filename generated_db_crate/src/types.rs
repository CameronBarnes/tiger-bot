// This file was generated with `clorinde`. Do not modify.

#[cfg(all(feature = "chrono", not(feature = "time")))]
pub mod time {
    pub type Timestamp = chrono::NaiveDateTime;
    pub type TimestampTz = chrono::DateTime<chrono::FixedOffset>;
    pub type Date = chrono::NaiveDate;
    pub type Time = chrono::NaiveTime;
}
#[cfg(all(feature = "time", not(feature = "chrono")))]
pub mod time {
    pub type Timestamp = time::PrimitiveDateTime;
    pub type TimestampTz = time::OffsetDateTime;
    pub type Date = time::Date;
    pub type Time = time::Time;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum QuoteType {
    Text,
    Document,
    Photo,
    Video,
    Voice,
}
impl<'a> postgres_types::ToSql for QuoteType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            QuoteType::Text => "Text",
            QuoteType::Document => "Document",
            QuoteType::Photo => "Photo",
            QuoteType::Video => "Video",
            QuoteType::Voice => "Voice",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "quote_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Text" => true,
                    "Document" => true,
                    "Photo" => true,
                    "Video" => true,
                    "Voice" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for QuoteType {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<QuoteType, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Text" => Ok(QuoteType::Text),
            "Document" => Ok(QuoteType::Document),
            "Photo" => Ok(QuoteType::Photo),
            "Video" => Ok(QuoteType::Video),
            "Voice" => Ok(QuoteType::Voice),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "quote_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Text" => true,
                    "Document" => true,
                    "Photo" => true,
                    "Video" => true,
                    "Voice" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
