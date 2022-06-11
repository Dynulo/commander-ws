pub enum Root {
    Version,
    Unauthenticated(Unauthenticated),

}

pub enum Unauthenticated {
    Login(String),
}

