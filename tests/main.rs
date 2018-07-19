extern crate rvk;

use rvk::{methods::*, APIClient, Params};

#[test]
fn users_get_wrong_token() {
    let api = APIClient::new("wrong-token".into());

    let mut params = Params::new();
    params.insert("user_ids".into(), "1".into());

    let res = users::get(&api, params);

    match res {
        Ok(_) => unreachable!(),
        Err(_) => (),
    };
}
