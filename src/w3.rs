use lazy_static::lazy_static;

pub type Transport = web3::Web3<web3::transports::http::Http>;

lazy_static! {
    pub static ref WEB3: Transport = {
        let web3_url = option_env!("WEB3_URL");
        println!("W3 URL is set to {:?}", web3_url);

        let web3_url = web3_url.unwrap_or("http://localhost:8545");

        let transport = web3::transports::Http::new(web3_url).unwrap();
        return web3::Web3::new(transport);
    };
}

pub fn init() {
    lazy_static::initialize(&WEB3);
}
