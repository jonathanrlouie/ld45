/// General purpose utility types

// newtypes for making it easier to know what args should be passed into which positions
pub struct PngPath<'a>(pub &'a str);
pub struct RonPath<'a>(pub &'a str);