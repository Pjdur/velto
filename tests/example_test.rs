//! This is an example of how to write tests using Velto's `TestRequest` utility.
//! It demonstrates how to define routes and assert responses in a test context.

use velto::test::TestRequest;
use velto::{route, App, Response};

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
