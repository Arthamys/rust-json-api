use crate::schema::revisions;
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Revision {
    pub id: i32,
    pub content: String,
    pub creation_time: NaiveDateTime,
    pub document_id: i32,
}

impl Revision {
    pub fn to_json(self) -> RevisionJson {
        RevisionJson {
            id: self.id,
            content: self.content,
            creation_time: self.creation_time,
        }
    }
}

// Used as a Data Transfer Object, omitting internal id
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct RevisionJson {
    pub id: i32,
    pub content: String,
    pub creation_time: NaiveDateTime,
}

// Data Access Object
#[derive(Insertable)]
#[table_name = "revisions"]
pub struct NewRevision<'a> {
    pub content: &'a str,
    pub creation_date: &'a NaiveDateTime,
    pub document_id: i32,
}
