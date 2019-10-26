pub mod request {
    enum HTTPMethod {
        GET,
        POST,
        PUT,
        DELETE
    }

    struct HTTPRequest {
        method: HTTPMethod,
    }
}