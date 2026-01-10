use std::process::{ExitCode, Termination};

#[repr(u8)]
pub enum EC {
    Good = 0,
    Bad = 1,
    Skip = 125,
    Abort = 255,
}

impl Termination for EC {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

fn main() -> EC {
    std::panic::catch_unwind(|| todo!("test the commit")).unwrap_or(EC::Skip)
}

// echo $?
// 125
