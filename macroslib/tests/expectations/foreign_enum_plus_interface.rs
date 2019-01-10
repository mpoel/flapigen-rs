foreign_enum!(
    enum ControlItem {
        GNSS = ControlItem::GnssWorking,
        GPS_PROVIDER = ControlItem::AndroidGPSOn,
    }
);

foreign_interface!(interface ControlStateObserver {
    self_type ControlStateChange;
    onSessionUpdate = ControlStateChange::on_state_changed(&self, item: ControlItem, is_ok: bool);
});
