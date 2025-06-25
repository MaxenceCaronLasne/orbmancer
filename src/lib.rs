#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

#[cfg(test)]
#[agb::entry]
fn main(mut _gba: agb::Gba) -> ! {
    test_main();
    loop {
        agb::display::busy_wait_for_vblank();
    }
}

#[cfg(test)]
mod tests {
    use agb::Gba;

    #[test_case]
    fn test_sample(_gba: &mut Gba) {}
}
