pub struct Red(pub String);

impl std::fmt::Display for Red {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.0)
    }
}

pub struct Blue(pub String);

impl std::fmt::Display for Blue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[34m{}\x1b[0m", self.0)
    }
}

pub struct Gray(pub String);

impl std::fmt::Display for Gray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[90m{}\x1b[0m", self.0)
    }
}
