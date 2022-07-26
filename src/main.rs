use tide::prelude::*;
use tide::{Request, Response, StatusCode};

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn get_seperated_names(&self) -> Result<Vec<&str>, &str> {
        Ok(self.name.split(' ').collect::<Vec<&str>>())
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").post(homepage);
    println!("[GitMD] Listening on http://127.0.0.1:8088");
    app.listen("127.0.0.1:8088").await?;
    Ok(())
}

async fn homepage(mut req: Request<()>) -> tide::Result {
    println!("[GitMD] Request received");
    match req.body_json::<User>().await {
        Ok(user) => {
            println!("[GitMD] {:?}", user);
            let names = user.get_seperated_names().unwrap();
            println!("[GitMD] Names: {} | Age: {}", names.join(" "), user.age);
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(format!("Hello, {}! Age: {}", names[0], user.age));
            Ok(res)
        }
        Err(error) => {
            println!("[GitMD] No user found");
            println!("[GitMD] Error: {}", error);
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(format!("{:?}", error));
            Ok(res)
        }
    }
}
