use crate::schema::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String,
    pub is_enabled: Option<bool>,

}

impl User {
    pub fn check_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}

//we'll insert data using the ORM so I need to create a predefined
//struct for it
#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    username: &'a str,
    password: String,
    is_enabled: Option<bool>,
}

//could be changed later on with a builder method...
pub fn new_user<'a>(username: &'a str, password: &str) -> NewUser<'a> {
    NewUser {
        username,
        password: bcrypt::hash(password, 5).unwrap(),
        is_enabled: None,
    }
}


