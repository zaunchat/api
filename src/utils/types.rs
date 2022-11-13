use serde::{Deserialize, Serialize};
use sqlx::{encode::IsNull, error::BoxDynError, postgres::*, Decode, Encode, Postgres, Type};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Private<T: Sized> {
    Hidden(T),
    Shown(T),
}

impl<T: Type<Postgres>> Type<Postgres> for Private<T> {
    fn type_info() -> PgTypeInfo {
        T::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        T::compatible(ty)
    }
}

impl<'a, T: Encode<'a, Postgres>> Encode<'_, Postgres> for Private<T> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        T::encode_by_ref(self, buf)
    }
}

impl<'r, T: Decode<'r, Postgres>> Decode<'r, Postgres> for Private<T> {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        // By default the value is hidden
        T::decode(value).map(|v| Self::Hidden(v))
    }
}

impl<T> From<T> for Private<T> {
    fn from(d: T) -> Self {
        Self::Hidden(d)
    }
}

impl<T: Default> Default for Private<T> {
    fn default() -> Self {
        Self::Hidden(T::default())
    }
}

impl<T: Clone> Private<T> {
    pub fn is_private(&self) -> bool {
        matches!(self, Private::Hidden(_))
    }

    pub fn set_public(&mut self) {
        *self = Self::Shown((**self).clone());
    }

    pub fn set_private(&mut self) {
        *self = Self::Hidden((**self).clone());
    }
}

impl<T> std::ops::Deref for Private<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hidden(x) | Self::Shown(x) => x,
        }
    }
}

impl<T> std::ops::DerefMut for Private<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hidden(x) | Self::Shown(x) => x,
        }
    }
}

impl_opg_model!(generic_simple: Private<T>);
