use crate::schema::documents;

#[derive(Queryable, Serialize, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Serialize, Debug)]
#[table_name = "documents"]
pub struct NewDocument {
    pub title: String,
}
