use ntex::web::HttpRequest;

pub fn parse_key_from_request(req: &HttpRequest) -> String {
    // TODO: handle error
    let key: String = req.match_info().get("key").unwrap().parse().unwrap();
    key
}
