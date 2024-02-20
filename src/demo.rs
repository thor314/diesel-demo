// use dotenvy::dotenv;
use std::env;

use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use shuttle_secrets::SecretStore;

/// establish a database connection
pub async fn establish_connection(secrets: &SecretStore) -> AsyncPgConnection {
  let db_url = secrets.get("DATABASE_URL").expect("DATABASE_URL must be set");

  AsyncPgConnection::establish(&db_url)
    .await
    .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

mod models {
  use diesel::prelude::*;

  use crate::schema::posts;

  // queryable will generate the code to load a Post from an SQL query
  // Selectable will generate code to construct a matching select clause
  #[derive(Queryable, Selectable)]
  // match to a schema for selectable
  #[diesel(table_name = posts)]
  // use postgres, improve compiler error messages.
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct Post {
    // warning that field order should match the order of the table macro
    pub id:        i32,
    pub title:     String,
    pub body:      String,
    pub published: bool,
  }

  // insertable will generate the code to insert a new Post into the database
  #[derive(Insertable)]
  #[diesel(table_name = posts)]
  pub struct NewPost<'a> {
    pub title: &'a str,
    pub body:  &'a str,
  }
}

// lesson part one. These two lines are not always suggested, but fix many things:
// use diesel::prelude::*;
// use diesel_async::{AsyncConnection, RunQueryDsl};
pub mod show_posts {
  use diesel::prelude::*;
  use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

  use crate::demo::{establish_connection, models::*};

  pub async fn show_posts(conn: &mut AsyncPgConnection) {
    use crate::schema::posts::dsl::*;

    // demo: query for some posts
    let results: Vec<Post> = posts
      .filter(published.eq(true)) // where
      .limit(5)
      .select(Post::as_select())
      .load(conn)
      .await
      .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
      println!("{}", post.title);
      println!("-----------\n");
      println!("{}", post.body);
    }
  }
}

pub mod create_posts {
  use diesel::prelude::*;
  use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

  use crate::demo::models::{NewPost, Post};
  pub async fn create_post(conn: &mut AsyncPgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
      .values(&new_post)
      .returning(Post::as_returning())
      .get_result(conn)
      .await
      .expect("Error saving new post")
  }
}
