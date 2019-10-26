pub mod routing {
    struct Router {
        routes: vec<Route>
    }

    //noinspection ALL
    type RouteHandler = &fn(Request, Response) -> Response;

    struct Route(str, RouteHandler);

}