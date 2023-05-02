//! Contains the Entities and ValueObjects of the application.
//!
//! <h5 align="center">Domain Diagram</h5>
//!
//! <div align="center">
//!
//! [![](https://www.plantuml.com/plantuml/svg/ZP5FIyGm4CNl-HHprEA2VsxBMlRWGM7z0-wUIxSphWDDfamdoY9-TnDRbXOyc5DUtczUIDwwjrej-jQrz6PW0ig7TmDafSPOWdEzQMmCjeBoOEJXoq5QNTm3unAAOXSPY-BYTDfoRzNVbh6rb8rNKmw7gHN4VftdBj-w6e1oIdJAecp7JBsC-mrzK3k5b3m799r4tTL6M_QqsbbJpX9H0cLmBp8BQPJI35QrPNSsFyLYX__1J8_La7XsxhwTen0zp1umVkcJ189Rk1n6D_APnDKW7Wav-gIl1tsVIahg9kVDenQBybJLSjcDddDh2fXg_W40)](https://www.plantuml.com/plantuml/uml/ZP5FIyGm4CNl-HHprEA2VsxBMlRWGM7z0-wUIxSphWDDfamdoY9-TnDRbXOyc5DUtczUIDwwjrej-jQrz6PW0ig7TmDafSPOWdEzQMmCjeBoOEJXoq5QNTm3unAAOXSPY-BYTDfoRzNVbh6rb8rNKmw7gHN4VftdBj-w6e1oIdJAecp7JBsC-mrzK3k5b3m799r4tTL6M_QqsbbJpX9H0cLmBp8BQPJI35QrPNSsFyLYX__1J8_La7XsxhwTen0zp1umVkcJ189Rk1n6D_APnDKW7Wav-gIl1tsVIahg9kVDenQBybJLSjcDddDh2fXg_W40)
//!
//! </div>

/// Objects with a unique identity, and usually with state (e.g., a person).
pub mod entities {
    pub mod todo_item;
}

/// Immutable objects that are constructed when needed and discarded when no longer valuable
pub mod value_objects {
    pub mod priority_level;
}
