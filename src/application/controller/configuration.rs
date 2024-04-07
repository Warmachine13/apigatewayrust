use crate::infra::file_system::read_user_from_file;

pub async fn configuration() -> String {
    let routes = read_user_from_file("routes.json").unwrap();
    let mounted = serde_json::to_string(&routes).unwrap();
    return mounted;
}
