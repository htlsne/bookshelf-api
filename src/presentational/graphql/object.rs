use async_graphql::Enum;
use async_graphql::{InputObject, SimpleObject, ID};

use crate::domain;
use crate::use_case::dto::author::AuthorDto;
use crate::use_case::dto::author::CreateAuthorDto;
use crate::use_case::dto::book::BookDto;

#[derive(SimpleObject)]
pub struct User {
    id: ID,
}

impl User {
    pub fn new(id: ID) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum BookFormat {
    EBook,
    Printed,
    Unknown,
}

impl From<domain::entity::book::BookFormat> for BookFormat {
    fn from(book_format: domain::entity::book::BookFormat) -> Self {
        use domain::entity::book::BookFormat as DomainBookFormat;
        match book_format {
            DomainBookFormat::EBook => BookFormat::EBook,
            DomainBookFormat::Printed => BookFormat::Printed,
            DomainBookFormat::Unknown => BookFormat::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum BookStore {
    Kindle,
    Unknown,
}

impl From<domain::entity::book::BookStore> for BookStore {
    fn from(book_format: domain::entity::book::BookStore) -> Self {
        use domain::entity::book::BookStore as DomainBookStore;
        match book_format {
            DomainBookStore::Kindle => BookStore::Kindle,
            DomainBookStore::Unknown => BookStore::Unknown,
        }
    }
}

#[derive(SimpleObject)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author_ids: Vec<String>,
    pub isbn: String,
    pub read: bool,
    pub owned: bool,
    pub priority: i32,
    pub format: BookFormat,
    pub store: BookStore,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Book {
    pub fn new(
        id: String,
        title: String,
        author_ids: Vec<String>,
        isbn: String,
        read: bool,
        owned: bool,
        priority: i32,
        format: BookFormat,
        store: BookStore,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            title,
            author_ids,
            isbn,
            read,
            owned,
            priority,
            format,
            store,
            created_at,
            updated_at,
        }
    }
}

impl From<BookDto> for Book {
    fn from(book_dto: BookDto) -> Self {
        Self {
            id: book_dto.id,
            title: book_dto.title,
            author_ids: book_dto.author_ids,
            isbn: book_dto.isbn,
            read: book_dto.read,
            owned: book_dto.owned,
            priority: book_dto.priority,
            format: book_dto.format.into(),
            store: book_dto.store.into(),
            created_at: book_dto.created_at.unix_timestamp(),
            updated_at: book_dto.created_at.unix_timestamp(),
        }
    }
}

#[derive(SimpleObject)]
pub struct Author {
    pub id: ID,
    pub name: String,
}

impl Author {
    pub fn new(id: String, name: String) -> Self {
        Self { id: ID(id), name }
    }
}

impl From<AuthorDto> for Author {
    fn from(author: AuthorDto) -> Self {
        let AuthorDto { id, name } = author;
        Author::new(id, name)
    }
}

#[derive(InputObject)]
pub struct CreateAuthorInput {
    pub name: String,
}

impl CreateAuthorInput {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Into<CreateAuthorDto> for CreateAuthorInput {
    fn into(self) -> CreateAuthorDto {
        CreateAuthorDto::new(self.name)
    }
}
