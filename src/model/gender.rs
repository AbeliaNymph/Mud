///# 性别
/// ---
/// ## 成员
/// - 男
/// - 女
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn default() -> Gender {
        Gender::Male
    }

    pub fn display(&self) {
        match self {
            Gender::Male => {
                println!("Gender: Male");
            }

            Gender::Female => {
                println!("Gender: Female");
            }
        }
    }
}