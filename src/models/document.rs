use crate::schema::documents;

#[derive(Queryable, Serialize, Debug)]
pub struct Document {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "documents"]
pub struct NewDocument<'a> {
    pub title: &'a str,
}
