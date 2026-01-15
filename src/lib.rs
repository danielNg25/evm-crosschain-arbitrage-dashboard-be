pub mod bot;
pub mod config;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod routes;
pub mod services;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opportunity_status_display() {}
}
