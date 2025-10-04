use std::fmt;
use std::io;


#[derive(Debug)]
pub enum ProgramError {

    InOut(io::Error),
    Construct(String),
    //Parsing(String),
    //UnknownCommand(String),
    //ExecutionFail { cmd: String, code: Option<i32>}

}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            ProgramError::InOut(e) => write!(f,"-Input/Output error: {}", e),
            ProgramError::Construct(cnst) => write!(f, "-Session construction error: {}", cnst),
            //ProgramError::Parsing(msg) => write!(f, "-Parsing error: {}", msg),
            //ProgramError::UnknownCommand(cmd) => write!(f, "-Command not found: {}", cmd),
            /*ProgramError::ExecutionFail { cmd, code } => {
                if let Some(code) = code {
                    write!(f, "-Command '{}' exited with status: {}", cmd, code)
                } else {
                    write!(f, "-Command '{}' failed without status code", cmd)
                }
            }*/
        }

    }
}

impl std::error::Error for ProgramError {
    // Optionally override source() if variants wrap underlying errors
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProgramError::InOut(e) => Some(e),
            _ => None,
        }
    }
}

// Convenience alias
pub type ProgResult<T> = Result<T, ProgramError>;

impl From<io::Error> for ProgramError {
    fn from(e: io::Error) -> Self {
        ProgramError::InOut(e)
    }
}


