pub struct FileContext {
    name: String,
    extension: String,
    path: String,

    enumerations: Option<FileEnumeration>
}

pub struct FileEnumeration {
    language: Option<String>,
    platform: Option<String>,
    use_enumeration: Option<String>
}