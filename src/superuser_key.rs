use db_schema::superuser_keys;

#[derive(Queryable, Debug)]
pub struct SuperuserKey {
    pub public_key: String,
}

#[derive(Insertable, Debug)]
#[table_name = "superuser_keys"]
pub struct NewSuperuserKey {
    pub public_key: String,
}
