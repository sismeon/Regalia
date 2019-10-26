use std::net::TCPListener;

pub mod server {

    struct Server {
        port: u16,
        router: routing::Router
    }

    impl Server {
        fn listen(&self){
            let listener = TCPListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
            println!("Listening on port {}", self.port);

            for stream in listener.incoming(){
                let stream = stream.unwrap();

                //let request: request::HTTPRequest = request::parse_HTTPRequest();
                //let route_handler: routing::RouteHandler = self.router.get_route(request.url);
                //route_handler(request);
                // TODO implement a thread pool after initial testing
                println!("Connection received!");
            }
        }
    }

    struct Settings {
        port: Option<u16>,
        router: Option<routing::Router>
    }

    fn server_from_settings(settings: settings::Settings) -> Server {
        //todo
    }

    fn create_default_server() -> Server {
        return Server {
            port: 3000,
            router: routing::create_default_router()
        }
    }

    fn new(settings: Option<settings::Settings>) -> Server {
        match settings {
            Some(set) => server_from_settings(set),
            None => create_default_server()
        }
    }

}