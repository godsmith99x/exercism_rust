// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 if self.level < 10 => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            0 if self.level >= 10 => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match (self.mana, mana_cost) {
            (None, mc) => {
                if self.health >= mc {
                    self.health -= mc;
                } else {
                    self.health = 0;
                }
                0
            }
            (Some(m), mc) if m < mc => 0,
            (Some(m), mc) if m >= mc => {
                self.mana = Some(m - mc);
                2 * mc
            }
            (Some(_), _) => 0,
        }
    }
}
