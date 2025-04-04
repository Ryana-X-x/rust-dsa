fn filter_request<F>(request: &str, filter: F) -> bool
where 
    F: Fn(&str) -> bool 
{
    filter(request)
}

fn main() {
    let is_admin_route = |route: &str| route.starts_with("/admin") ;

    let request1 = "/admin/dashboard" ;
    let request2 = "/user/profile" ;

    println!(
        "Is '{}' an admin route? {}", 
        request1,
        filter_request(request1, is_admin_route) 
    ) ;

    println!(
        "Is '{}' an admin route? {}", 
        request2, 
        filter_request(request2, is_admin_route)
    ) ;
}
