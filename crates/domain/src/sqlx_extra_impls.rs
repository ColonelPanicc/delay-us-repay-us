use anyhow::{Context as _, Error, Result};
use sqlx::{Database, Decode, Encode, Type, encode::IsNull, error::BoxDynError};

use crate::RailwayStationId;

impl<DB: Database> Type<DB> for RailwayStationId
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }
}

impl<'q, DB: Database> Encode<'q, DB> for RailwayStationId
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        let s: String = self.0.iter().collect();
        s.encode_by_ref(buf)
    }
}

impl<'q, DB: Database> Decode<'q, DB> for RailwayStationId
where
    String: Decode<'q, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'q>) -> Result<Self, BoxDynError> {
        let s = String::decode(value)?;
        string_to_railway_station_id(&s).map_err(Error::into_boxed_dyn_error)
    }
}

pub(crate) fn string_to_railway_station_id(s: &str) -> Result<RailwayStationId> {
    if s.len() != 3 {
        anyhow::bail!("expected length 3 but string was {s}");
    }

    let mut chars = s.chars();
    let arr = [
        chars.next().context("1st char missing after len check")?,
        chars.next().context("2nd char missing after len check")?,
        chars.next().context("3rd char missing after len check")?,
    ];
    Ok(RailwayStationId(arr))
}
