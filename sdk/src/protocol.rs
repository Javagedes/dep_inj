pub trait Protocol {
    fn guid() -> &'static r_efi::efi::Guid;
}

impl Protocol for r_efi::protocols::rng::Protocol {
    fn guid() -> &'static r_efi::efi::Guid {
        &r_efi::protocols::rng::PROTOCOL_GUID
    }
}

impl Protocol for r_efi::protocols::udp4::Protocol {
    fn guid() -> &'static r_efi::efi::Guid {
        &r_efi::protocols::udp4::PROTOCOL_GUID
    }
}
