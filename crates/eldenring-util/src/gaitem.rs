use std::intrinsics::transmute;
use eldenring::cs::{CSGaitemImp, CSGaitemIns, ChrIns, GaitemHandle, GaitemLookupResult, ItemCategory, SwordArtsParamLookupResult};
use pelite::pe64::Pe;
use crate::program::Program;
use crate::rva;

pub trait CSGaitemImpExt {
    fn gaitem_ins_by_handle(&self, handle: &GaitemHandle) -> Option<&CSGaitemIns>;

    fn gaitem_ins_by_handle_mut(&mut self, handle: &GaitemHandle) -> Option<&mut CSGaitemIns>;
}

impl CSGaitemImpExt for CSGaitemImp {
    fn gaitem_ins_by_handle(&self, handle: &GaitemHandle) -> Option<&CSGaitemIns> {
        // Can't do a lookup for a handle that is not supposed to be in here anyway.
        if !handle.is_indexed() {
            return None;
        }

        let index = handle.index() as usize;
        if index > self.gaitems.len() {
            return None;
        }

        Some(self.gaitems[index].as_ref()?.as_ref())
    }

    fn gaitem_ins_by_handle_mut(&mut self, handle: &GaitemHandle) -> Option<&mut CSGaitemIns> {
        // Can't do a lookup for a handle that is not supposed to be in here anyway.
        if !handle.is_indexed() {
            return None;
        }

        let index = handle.index() as usize;
        if index > self.gaitems.len() {
            return None;
        }

        Some(self.gaitems[index].as_mut()?.as_mut())
    }
}

pub trait GaitemLookupResultExt {
    fn get_gaitem_ins_by_category(&self, handle: *const GaitemHandle, item_category: ItemCategory) -> Option<&CSGaitemIns>;

    fn get_sword_arts_param_id_for_weapon(&mut self) -> Option<i32>;
}

impl GaitemLookupResultExt for GaitemLookupResult {
    fn get_gaitem_ins_by_category(&self, handle: *const GaitemHandle, item_category: ItemCategory) -> Option<&CSGaitemIns> {
        let rva = Program::current()
            .rva_to_va(rva::get().gaitem_lookup_result_get_gaitem_ins_by_category)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&GaitemLookupResult, *const GaitemHandle, ItemCategory) -> Option<&CSGaitemIns>>(rva) };
        call(self, handle, item_category)
    }

    fn get_sword_arts_param_id_for_weapon(&mut self) -> Option<i32> {
        let rva = Program::current()
            .rva_to_va(rva::get().gaitem_get_swordarts_param_id_for_weapon)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&GaitemLookupResult) -> Option<i32>>(rva) };
        call(self)
    }
}