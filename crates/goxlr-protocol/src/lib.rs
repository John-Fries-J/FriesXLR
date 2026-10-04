//! Minimal protocol identifiers used by FriesXLR Phase 1.
//!
//! These constants are adapted from the MIT-licensed GoXLR Utility project:
//! https://github.com/GoXLR-on-Linux/goxlr-utility
//! See `NOTICE.md` for attribution.

use goxlr_model::DeviceModel;

pub const TC_HELICON_VENDOR_ID: u16 = 0x1220;
pub const GOXLR_PRODUCT_ID: u16 = 0x8fe0;
pub const GOXLR_MINI_PRODUCT_ID: u16 = 0x8fe4;

pub fn model_from_product_id(product_id: u16) -> DeviceModel {
    match product_id {
        GOXLR_PRODUCT_ID => DeviceModel::GoXlr,
        GOXLR_MINI_PRODUCT_ID => DeviceModel::GoXlrMini,
        _ => DeviceModel::Unknown,
    }
}

pub fn is_known_goxlr_device(vendor_id: u16, product_id: u16) -> bool {
    vendor_id == TC_HELICON_VENDOR_ID
        && matches!(product_id, GOXLR_PRODUCT_ID | GOXLR_MINI_PRODUCT_ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_product_ids_to_models() {
        assert_eq!(model_from_product_id(GOXLR_PRODUCT_ID), DeviceModel::GoXlr);
        assert_eq!(
            model_from_product_id(GOXLR_MINI_PRODUCT_ID),
            DeviceModel::GoXlrMini
        );
        assert_eq!(model_from_product_id(0), DeviceModel::Unknown);
    }
}
