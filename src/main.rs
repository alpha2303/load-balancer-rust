mod lb;
mod thread_pool;

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let load_balancer: lb::LoadBalancer = lb::LoadBalancer::new(addr).unwrap();
    if let Err(e) = load_balancer.start() {
        eprintln!("Failed to start load balancer: {}", e);
    }
}
