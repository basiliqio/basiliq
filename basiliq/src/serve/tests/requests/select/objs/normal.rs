use super::*;

crate::run_test_request!(get_all, Method::GET, "/public__peoples", 200);
