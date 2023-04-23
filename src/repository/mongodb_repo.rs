use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
   bson::{extjson::de::Error, oid::ObjectId, doc},
   results::{ InsertOneResult, UpdateResult},
   Client, Collection, options::ClientOptions,
};
use futures::stream::TryStreamExt;
use crate::models::user_model::User;


pub struct MongoRepo {
   col: Collection<User>,
}

impl MongoRepo {
   pub async fn init() -> Self {
      dotenv().ok();
      /*let uri = match env::var("MONGOURI") {
         Ok(v) => v.to_string(),
         Err(_) => format!("Error loading env variable"),
      };*/

      /*let uri = "mongodb://localhost:27017/";
      let mut options = ClientOptions::parse(uri).await.unwrap();
      options.direct_connection = Some(true);
      let client = Client::with_options(options).unwrap();
      */
      
      let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
      let db = client.database("rustDB");
      let col: Collection<User> = db.collection("User");
      MongoRepo { col}
   }

   pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
      let new_doc = User {
         id: None,
         name: new_user.name,
         location: new_user.location,
         title: new_user.title,
      };

      let user = self
      .col
      .insert_one(new_doc, None)
         .await
      .ok()
         .expect("Error creating user");

      Ok(user)
   }

   pub async fn get_user(&self, id: &String) -> Result<User, Error> {
      let obj_id = ObjectId::parse_str(id).unwrap();
      let filter = doc! {"_id": obj_id};
      let user_detail = self
         .col
         .find_one(filter, None)
         .await
         .ok()
         .expect("Error getting user's detail");
      Ok(user_detail.unwrap())
   }

   pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
      let obj_id = ObjectId::parse_str(id).unwrap();
      let filter = doc! {"_id": obj_id};

      let new_doc = doc! {
         "$set":
         {
            "id": new_user.id,
            "name": new_user.name,
            "location": new_user.location,
            "title": new_user.title
         },
      };

      let update_doc = self
         .col
         .update_one(filter, new_doc, None)
         .await
         .ok()
         .expect("Error updating User");
      Ok(update_doc)

   }
   pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }

}
