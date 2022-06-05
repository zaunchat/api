lazy_static! {
    static ref CONNECTION: Rbatis = Rbatis::new();
}

pub async fn connect() {
    CONNECTION.link().await.unwrap();
}

pub fn query() {

}

pub fn get() {
    CONNECTION
}