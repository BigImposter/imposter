use yew::AttrValue;

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub name: AttrValue,
}

impl Player {
    pub fn new(name: AttrValue) -> Player {
        Player { name: name}
    }
}