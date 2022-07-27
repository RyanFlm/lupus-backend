use super::*;

#[test]
pub fn get() {
    let rocket = rocket::build().mount("/", routes::index::routes());
    let client = Client::tracked(rocket).unwrap();

    let req = client.get("/").header(ContentType::JSON);
    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok, "Status not OK");
    assert_eq!(
        res.content_type(),
        Some(ContentType::JSON),
        "Content-Type not JSON"
    );

    let body = res.into_json::<Response>().unwrap();
    assert_eq!(body.success, true, "body.success not true");
}
