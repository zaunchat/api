use tower_http::compression::CompressionLayer;

pub fn handle() -> CompressionLayer {
    CompressionLayer::new()
}
