use super::health_check::HealthCheck;
use super::server::Server;

#[derive(Debug)]
pub struct RouteScheduler {
    health_check: HealthCheck,
}

impl RouteScheduler {
    pub fn new(server_host_list: Vec<&'static str>) -> Self {
        RouteScheduler {
            health_check: HealthCheck::new(server_host_list),
        }
    }

    pub fn get_next_server(&mut self) -> Option<&'static str> {
        // self.health_check.check();
        // let healthy_servers = self.health_check.get_healthy_servers();
        todo!()
    }
}
