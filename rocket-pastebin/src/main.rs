#[macro_use]
extern crate rocket;

mod paste_id;
#[cfg(test)]
mod tests;

use std::io;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::Plain;
use rocket::tokio::fs::{self, File};

use crate::paste_id::PasteID;

const HOST: Absolute<'static> = uri!("http://127.0.0.1:8000");

const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteID::new(ID_LENGTH);
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[get("/<id>")]
async fn retrieve(id: PasteID<'_>) -> Option<Plain<File>> {
    File::open(id.file_path()).await.map(Plain).ok()
}

#[delete("/<id>")]
async fn delete(id: PasteID<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://127.0.0.1:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("//", routes![index, upload, delete, retrieve])
}
