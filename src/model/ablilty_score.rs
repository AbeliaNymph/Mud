///# 能力值
/// ---
/// ## 成员
/// - 体力
/// - 健康
#[derive(Debug)]
pub struct AbilityScore {
    energy: u32,
    health: u32,
}

impl AbilityScore {
    pub fn new(energy: u32, health: u32) -> AbilityScore {
        AbilityScore {
            energy,
            health,
        }
    }

    pub fn display(&self) {
        println!("Energy: {}/{}", self.energy, 270);
        println!("Health: {}/{}", self.health, 100);
    }


}