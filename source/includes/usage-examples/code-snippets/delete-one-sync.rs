use mongodb::{ bson::doc, sync::{ Client, Collection } };
use bson::Document;

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri)?;

    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter =
        doc! { "$and": [
           doc! { "name": "Haagen-Dazs" },
           doc! { "borough": "Brooklyn" }
       ]
    };

    let result = my_coll.delete_one(filter, None)?;

    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}
