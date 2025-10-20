use velto::{App, route, Response};
use velto::test::TestRequest;

#[test]
fn test_homepage() {
    let mut app = App::new();
    route!(app, "/" => |_req| {
        Response::from_string("Hello, test!")
    });

    let res = TestRequest::new("GET", "/").send(&app);
    assert_eq!(res.status_code(), 200);
    assert!(res.body().contains("Hello"));
}
