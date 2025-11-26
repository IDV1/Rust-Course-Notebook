fn main() {
    let user_james = User{
        username: "dog".to_string(),
        email: "12345@lima.com".to_string(),
        sign_in_count: 1
    };

    dbg!(&user_james);
    return_marios();
}
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
}

trait UserTrait{}

impl UserTrait for User{}

fn return_marios() -> impl UserTrait {
    User{
        username: "Mario".to_string(),
        email: "99996666@mm.com".to_string(),
        sign_in_count: 2
    }
}
