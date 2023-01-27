// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: (self.level >= 10).then_some(100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mana = self.mana.unwrap_or_else(|| {
            self.health = self.health.saturating_sub(mana_cost);
            0
        });

        // We can reach this line even if self.mana is None but it is still okay,
        // as we will not proceed any further (except for scenarios, where mana_cost=0).
        // That's why I've added a check `mana == 0`. In this task it's not really required,
        // but in real life I prefer to bail out as early as possible.
        if (mana == 0 || mana < mana_cost) {
            return 0;
        }

        self.mana = Some(mana.saturating_sub(mana_cost));
        mana_cost * 2
    }
}
