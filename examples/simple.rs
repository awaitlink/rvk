use rvk::{methods::users, objects::user::User, APIClient, Params};

#[tokio::main]
async fn main() {
    let api = APIClient::new("your_access_token"); // Create an API Client

    let mut params = Params::new(); // Create a HashMap to store parameters
    params.insert("user_ids".into(), "1".into());

    let res = users::get::<Vec<User>>(&api, params).await;

    match res {
        Ok(users) => {
            let user: &User = &users[0];

            println!(
                "User #{} is {} {}.",
                user.id, user.first_name, user.last_name
            );
        }
        Err(e) => println!("{}", e),
    };
}
