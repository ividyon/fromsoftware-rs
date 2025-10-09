use eldenring::cs::{ChrAsm, ChrAsmSlot, ChrIns, GaitemHandle};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChrAsmError {
    #[error("Invalid slot index")]
    InvalidIndex,
}
pub trait ChrAsmExt {
    fn get_selected_right_weapon(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_selected_left_weapon(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_selected_right_arrow(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_selected_left_arrow(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_selected_right_bolt(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_selected_left_bolt(&mut self) -> Result<GaitemHandle, ChrAsmError>;
    fn get_right_weapon_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError>;
    fn get_left_weapon_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError>;
    fn get_arrow_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError>;
    fn get_bolt_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError>;
    fn get_sword_arts_param_id_from_weapon(&mut self, weapon: GaitemHandle) -> Result<i32, ChrAsmError>;
}

impl ChrAsmExt for ChrAsm {
    fn get_selected_right_weapon(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_right_weapon_by_slot(self.equipment.selected_slots.right_weapon_slot)
    }
    fn get_selected_left_weapon(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_right_weapon_by_slot(self.equipment.selected_slots.left_weapon_slot)
    }
    fn get_selected_right_arrow(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_arrow_by_slot(self.equipment.selected_slots.right_arrow_slot)
    }
    fn get_selected_left_arrow(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_arrow_by_slot(self.equipment.selected_slots.left_arrow_slot)
    }
    fn get_selected_right_bolt(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_arrow_by_slot(self.equipment.selected_slots.right_bolt_slot)
    }
    fn get_selected_left_bolt(&mut self) -> Result<GaitemHandle, ChrAsmError> {
        self.get_arrow_by_slot(self.equipment.selected_slots.left_bolt_slot)
    }
    fn get_right_weapon_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError> {
        if slot > 2 {
            return Err(ChrAsmError::InvalidIndex);
        };
        Ok(self.gaitem_handles[(ChrAsmSlot::WeaponRight1 as u32 + (slot * 2)) as usize])
    }
    fn get_left_weapon_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError> {
        if slot > 2 {
            return Err(ChrAsmError::InvalidIndex);
        };
        Ok(self.gaitem_handles[(ChrAsmSlot::WeaponLeft1 as u32 + (slot * 2)) as usize])
    }
    fn get_arrow_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError> {
        if slot > 1 {
            return Err(ChrAsmError::InvalidIndex);
        };
        Ok(self.gaitem_handles[(ChrAsmSlot::Arrow1 as u32 + (slot * 2)) as usize])
    }
    fn get_bolt_by_slot(&mut self, slot: u32) -> Result<GaitemHandle, ChrAsmError> {
        if slot > 1 {
            return Err(ChrAsmError::InvalidIndex);
        };
        Ok(self.gaitem_handles[(ChrAsmSlot::Bolt1 as u32 + (slot * 2)) as usize])
    }

    fn get_sword_arts_param_id_from_weapon(&mut self, weapon: GaitemHandle) -> Result<i32, ChrAsmError> {
        todo!()
    }
}