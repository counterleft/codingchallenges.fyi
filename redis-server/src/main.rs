fn main() -> std::io::Result<()> {
    redis_server::listen("127.0.0.1:6379")
}
