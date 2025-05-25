dotenv::dotenv().ok(); // Load .env
let url = env::var("ELEMENTS_RPC_URL").unwrap();
