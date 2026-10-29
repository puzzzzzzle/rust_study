use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ActionError {
    #[error("target not found")]
    TargetNotFound, 
    #[error("timeout")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_action_error() {
        let error = ActionError::TargetNotFound;
        println!("error: {:?}", error);
        assert_eq!(error, ActionError::TargetNotFound);
        assert_eq!(format!("{}", error), "target not found");
    }
}