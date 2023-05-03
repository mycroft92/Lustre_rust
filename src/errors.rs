
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
    ParseError ,
    CMDLineError ,
    FileNotFoundError,
    UnknownError
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos : Option<Loc>,
    pub msg : String
}

pub type LusRes<T> = Result<T, Error>;

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std ::fmt::Result {
        let res = match self {
            ErrorKind::ParseError =>  format!("ParseError:"),
            ErrorKind::CMDLineError => format!("CMDLineError: "),
            ErrorKind::FileNotFoundError => format!("FileNotFoundError"),
            ErrorKind::UnknownError      => format!("UnknownError"),
        };
        write!(f, "{}", res)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std ::fmt::Result {
        match &self.pos {
            Some(p) => write!(f, "{} @ {}: {}", self.kind, p, self.msg),
            None         => write!(f, "{} : {}", self.kind,  self.msg)
        }
    }
}

impl std::default::Default for Error {
    fn default() -> Self {
        Error {kind: ErrorKind::UnknownError, pos: None, msg: String::from("")}
    }
}