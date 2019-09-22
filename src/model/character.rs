use crate::model::{gender, ablilty_score};

///# 角色
/// ---
/// ## 成员
/// - 姓名
/// - 性别
/// - 能力值
#[derive(Debug)]
pub struct Character {
    name: String,
    gender: gender::Gender,
    abliity_score: ablilty_score::AbilityScore,
}

impl Character {
    pub fn new(name: String, gender: gender::Gender) -> Character {
        Character {
            name,
            gender,
            abliity_score: ablilty_score::AbilityScore::new(270_u32, 100_u32),
        }
    }

    pub fn diplay(&self) {
        println!("Name: {}", self.name);
        self.gender.display();
        self.abliity_score.display();
    }

}