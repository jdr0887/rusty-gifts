#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(shared::LoggedUser),
    Guest,
}

impl<'a> Default for Session {
    fn default() -> Self {
        Self::Guest
    }
}

impl<'a> Session {
    pub fn new(viewer: Option<shared::LoggedUser>) -> Self {
        match viewer {
            Some(viewer) => Self::LoggedIn(viewer),
            None => Self::Guest,
        }
    }
    pub fn viewer(&self) -> Option<&shared::LoggedUser> {
        match self {
            Self::LoggedIn(viewer) => Some(viewer),
            Self::Guest => None,
        }
    }
}
