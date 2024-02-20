// use dotenvy::dotenv;
use std::env;

use diesel::{pg::PgConnection, prelude::*};
use shuttle_secrets::SecretStore;

/// establish a database connection
pub fn establish_connection(secrets: &SecretStore) -> PgConnection {
  let db_url = secrets.get("DATABASE_URL").expect("DATABASE_URL must be set");

  PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

mod models {
  use diesel::prelude::*;

  // queryable will generate the code to load a Post from an SQL query
  // Selectable will generate code to construct a matching select clause
  #[derive(Queryable, Selectable)]
  // match to a schema for selectable
  #[diesel(table_name = crate::schema::posts)]
  // use postgres, improve compiler error messages.
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct Post {
  // warning that field order should match the order of the table macro
    pub id:        i32,
    pub title:     String,
    pub body:      String,
    pub published: bool,
  }
}

mod show_posts {
  use diesel::prelude::*;

  use crate::demo::{establish_connection, models::*};

  fn show_posts(secrets: &shuttle_secrets::SecretStore) {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection(secrets);
    let results = posts
      .filter(published.eq(true))
      .limit(5)
      .select(Post::as_select())
      .load(connection)
      .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
      println!("{}", post.title);
      println!("-----------\n");
      println!("{}", post.body);
    }
  }
}
