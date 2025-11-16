mod health_check;
mod lb;
mod thread_pool;
mod route_scheduler;

const SERVER_HOST_LIST: [&'static str; 1] = ["127.0.0.1:8081"];

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let load_balancer: lb::LoadBalancer =
        lb::LoadBalancer::new(addr, SERVER_HOST_LIST.to_vec()).unwrap();
    if let Err(e) = load_balancer.start() {
        eprintln!("Failed to start load balancer: {}", e);
    }
}
