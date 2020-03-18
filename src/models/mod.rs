pub mod document;
pub mod revision;

// for convinience re-export the document type
pub use document::{Document, NewDocument};
pub use revision::{Revision, RevisionJson, NewRevision};
