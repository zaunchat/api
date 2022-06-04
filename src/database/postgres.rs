lazy_static! {
    static ref DB:Rbatis = Rbatis::new();
}

pub async fn connect() {
    DB.link().await.unwrap();
}

pub fn query() {

}


pub fn postgres() {
    DB
}