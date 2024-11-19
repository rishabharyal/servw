struct Handler {
    config: Config,
    reqest: HttpRequest,
}

impl Handler {
    pub fn new(config: Config, reqest: HttpRequest) -> Self {
        Self {
            config,
            reqest,
        }
    }

    pub fn handle(&self) -> String {
        // TODO: Implement
        // check if config has a pass key
        if config.pass != "" {
            // pass to the fastcgi server
        }

        if config.lb != "" {
            // pass to the load balancer
        }

        return "HTTP/1.1 200 OK\n\n".to_string();
    }
}

