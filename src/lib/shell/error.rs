
#[derive(Debug)]
pub enum ShellError {
	/// Contains the shell name that could not be matched
	UnknownType(String),
	/// Shell which might be APHE or solid AP, but cannot be identified automatically
	NonDeterministic(String),
}