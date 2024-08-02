#[derive(Debug, Clone)]
pub enum Languages {
    EN,
    AR,
}

#[derive(Debug, Clone)]
pub struct Language {
    pub name: Languages,
    pub dict_file: String,
    pub start_char: char,
}

impl From<Languages> for Language {
    fn from(value: Languages) -> Self {
        let dict_file: String;
        let start_char: char;

        match value {
            Languages::EN => {
                start_char = 'a';
                dict_file = String::from("./dictionaries/en.txt")
            }

            Languages::AR => {
                start_char = 'ء';
                dict_file = String::from("./dictionaries/ar.txt")
            }
        };


        Self {
            name: value,
            dict_file,
            start_char,
        }
    }
}