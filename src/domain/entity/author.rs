use getset::Getters;
use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct AuthorId {
    #[getset(get = "pub")]
    pub id: Uuid,
}

impl TryFrom<&str> for AuthorId {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(value).map_err(|err| {
            DomainError::Validation(format!(
                r#"Failed to parse id "{}" as uuid. Message from uuid crate: {}"#,
                value,
                err.to_string()
            ))
        })?;
        Ok(AuthorId { id })
    }
}

impl From<Uuid> for AuthorId {
    fn from(uuid: Uuid) -> Self {
        AuthorId { id: uuid }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct AuthorName {
    #[getset(get = "pub")]
    pub name: String,
}

impl AuthorName {
    pub fn new(name: String) -> Result<AuthorName, DomainError> {
        Ok(AuthorName { name })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Author {
    #[getset(get = "pub")]
    pub id: AuthorId,
    #[getset(get = "pub")]
    pub name: AuthorName,
}

impl Author {
    pub fn new(id: AuthorId, name: AuthorName) -> Result<Author, DomainError> {
        Ok(Author { id, name })
    }
}
