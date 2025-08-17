#[derive(Display, Debug)]
pub enum Command {
    AppVersion,
    Name,
    DeviceList,
    Attach,
    Info,
    Boot,
    Reset,
    Menu,

    List,
    PutFile,
    GetFile,
    Rename,
    Remove,

    GetAddress,
}

#[derive(Display, Debug)]

#[derive(Display, Debug)]
#[allow(dead_code)]
pub enum Space {
    None,
    SNES,
    CMD,
}

#[derive(Debug)]
pub struct Infos {
    pub version: String,
    pub dev_type: String,
    pub game: String,
    pub flags: Vec<String>,
}
