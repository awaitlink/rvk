use rvk::Params;
use rvk_methods::users;
use rvk_objects::user::User;

#[tokio::main]
async fn main() {
    // Create an API client that uses the API version supported by `rvk_methods`.
    let api = rvk_methods::supported_api_client("your_access_token");

    // A HashMap to store parameters.
    let mut params = Params::new();
    params.insert("user_ids".into(), "1".into());

    // Use a type from `rvk_objects` as the result type.
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
