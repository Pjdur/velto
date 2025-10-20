#[cfg(test)]
mod tests {
    use velto::test::TestRequest;
    use velto::{render, route, App, Response};

    #[test]
    fn test_basic_route() {
        let mut app = App::new();
        route!(app, "/" => |_req| {
            Response::from_string("Hello, Velto!")
        });

        let res = TestRequest::new("GET", "/").send(&app);
        assert_eq!(res.status_code(), 200);
        assert!(res.body().contains("Velto"));
    }

    #[test]
    fn test_404_response() {
        let app = App::new();
        let res = TestRequest::new("GET", "/missing").send(&app);
        assert_eq!(res.status_code(), 404);
        assert!(res.body().contains("404"));
    }

    #[test]
    fn test_template_rendering() {
        let mut app = App::new();
        route!(app, "/greet" => |_req| {
            render!("index.html", {
                "title" => "Test Title",
                "message" => "Hello from test!"
            })
        });

        let res = TestRequest::new("GET", "/greet").send(&app);
        assert_eq!(res.status_code(), 200);
        assert!(res.body().contains("Test Title"));
        assert!(res.body().contains("Hello from test!"));
    }

    #[test]
    fn test_redirect() {
        use velto::response::redirect;

        let mut app = App::new();
        route!(app, "/go" => |_req| {
            redirect("/target")
        });

        let res = TestRequest::new("GET", "/go").send(&app);
        assert_eq!(res.status_code(), 302);
        assert!(res.headers().get("location").unwrap() == "/target");
    }

    #[test]
    fn test_method_matching() {
        use velto::http_method::Method;

        let mut app = App::new();
        route!(app, [GET, POST] "/multi" => |req| {
            Response::from_string(format!("Method: {}", req.method()))
        });

        let get_res = TestRequest::new("GET", "/multi").send(&app);
        assert_eq!(get_res.status_code(), 200);
        assert!(get_res.body().contains("GET"));

        let post_res = TestRequest::new("POST", "/multi")
            .with_body("data")
            .send(&app);
        assert_eq!(post_res.status_code(), 200);
        assert!(post_res.body().contains("POST"));
    }
}
