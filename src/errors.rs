
#[derive(Debug)]
pub struct Loc {
    pub fname: String,
    pub span: (usize, usize),
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) of {}", self.span.0, self.span.1, self.fname)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    //Notate parse errors
    ParseError,
    CMDLineError,
    FileNotFoundError,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    pos : Loc,
    msg : String
}

pub type LusRes<T> = Result<T, Error>;

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std ::fmt::Result {
        let res = match self {
            ErrorKind::ParseError => "ParseError",
            ErrorKind::CMDLineError => "CMDLineError",
            ErrorKind::FileNotFoundError => "FileNotFoundError",
        };
        write!(f, "{}", res)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std ::fmt::Result {
        write!(f, "{} @ {}: {}", self.kind, self.pos, self.msg)
    }
}

