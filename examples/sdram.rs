//! Kudos to Alex Norman and his libdaisy-rust.
//! <https://github.com/x37v/libdaisy-rust/blob/develop/examples/sdram.rs>

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;

use daisy::hal::delay::DelayExt;
use daisy::hal::fmc::FmcExt;
use daisy::hal::gpio::Speed;
use daisy::led::Led;
use daisy::pac;

use stm32_fmc::devices::as4c16m32msa_6;

// - entry point --------------------------------------------------------------

#[entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = daisy::Board::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let ccdr = daisy::board_freeze_clocks!(board, dp);
    let pins = daisy::board_split_gpios!(board, ccdr, dp);
    let mut led_user = daisy::board_split_leds!(pins).USER;

    // - RAM handling ---------------------------------------------------------

    let size = 64 * 1024 * 1024;

    // Configure and initialize SDRAM
    let region_base_address = {
        macro_rules! fmc_pins {
            ($($pin:expr),*) => {
                (
                    $(
                        $pin.into_push_pull_output()
                            .speed(Speed::VeryHigh)
                            .into_alternate::<12>()
                            .internal_pull_up(true)
                    ),*
                )
            };
        }

        let sdram_pins = fmc_pins! {
            pins.SDRAM.A0, pins.SDRAM.A1, pins.SDRAM.A2, pins.SDRAM.A3,
            pins.SDRAM.A4, pins.SDRAM.A5, pins.SDRAM.A6, pins.SDRAM.A7,
            pins.SDRAM.A8, pins.SDRAM.A9, pins.SDRAM.A10, pins.SDRAM.A11,
            pins.SDRAM.A12, pins.SDRAM.BA0, pins.SDRAM.BA1, pins.SDRAM.D0,
            pins.SDRAM.D1, pins.SDRAM.D2, pins.SDRAM.D3, pins.SDRAM.D4,
            pins.SDRAM.D5, pins.SDRAM.D6, pins.SDRAM.D7, pins.SDRAM.D8,
            pins.SDRAM.D9, pins.SDRAM.D10, pins.SDRAM.D11, pins.SDRAM.D12,
            pins.SDRAM.D13, pins.SDRAM.D14, pins.SDRAM.D15, pins.SDRAM.D16,
            pins.SDRAM.D17, pins.SDRAM.D18, pins.SDRAM.D19, pins.SDRAM.D20,
            pins.SDRAM.D21, pins.SDRAM.D22, pins.SDRAM.D23, pins.SDRAM.D24,
            pins.SDRAM.D25, pins.SDRAM.D26, pins.SDRAM.D27, pins.SDRAM.D28,
            pins.SDRAM.D29, pins.SDRAM.D30, pins.SDRAM.D31, pins.SDRAM.NBL0,
            pins.SDRAM.NBL1, pins.SDRAM.NBL2, pins.SDRAM.NBL3, pins.SDRAM.SDCKE0,
            pins.SDRAM.SDCLK, pins.SDRAM.SDNCAS, pins.SDRAM.SDNE0, pins.SDRAM.SDRAS,
            pins.SDRAM.SDNWE
        };

        let mut delay = cp.SYST.delay(ccdr.clocks);
        dp.FMC
            .sdram(
                sdram_pins,
                as4c16m32msa_6::As4c16m32msa {},
                ccdr.peripheral.FMC,
                &ccdr.clocks,
            )
            .init(&mut delay)
    };

    // Configure MPU for external SDRAM
    {
        let mpu = cp.MPU;
        let scb = &mut cp.SCB;
        // Refer to ARM®v7-M Architecture Reference Manual ARM DDI 0403
        // Version E.b Section B3.5
        const MEMFAULTENA: u32 = 1 << 16;

        // Disable and reset MPU
        unsafe {
            // Make sure outstanding transfers are done
            cortex_m::asm::dmb();

            scb.shcsr.modify(|r| r & !MEMFAULTENA);

            // Disable the MPU and clear the control register
            mpu.ctrl.write(0);
        }

        const REGION_NUMBER0: u32 = 0x00;
        const REGION_FULL_ACCESS: u32 = 0x03;
        const REGION_CACHEABLE: u32 = 0x01;
        const REGION_WRITE_BACK: u32 = 0x01;
        const REGION_ENABLE: u32 = 0x01;

        assert_eq!(
            size & (size - 1),
            0,
            "SDRAM memory region size must be a power of 2"
        );
        assert_eq!(
            size & 0x1F,
            0,
            "SDRAM memory region size must be 32 bytes or more"
        );
        fn log2minus1(sz: u32) -> u32 {
            for i in 5..=31 {
                if sz == (1 << i) {
                    return i - 1;
                }
            }
            panic!("Unknown SDRAM memory region size!");
        }

        //info!("SDRAM Memory Size 0x{:x}", log2minus1(size as u32));

        // Configure region 0
        //
        // Cacheable, outer and inner write-back, no write allocate. So
        // reads are cached, but writes always write all the way to SDRAM
        unsafe {
            mpu.rnr.write(REGION_NUMBER0);
            mpu.rbar.write((region_base_address as u32) & !0x1F);
            mpu.rasr.write(
                (REGION_FULL_ACCESS << 24)
                    | (REGION_CACHEABLE << 17)
                    | (REGION_WRITE_BACK << 16)
                    | (log2minus1(size as u32) << 1)
                    | REGION_ENABLE,
            );
        }

        // Enable MPU
        const MPU_ENABLE: u32 = 0x01;
        const MPU_DEFAULT_MMAP_FOR_PRIVILEGED: u32 = 0x04;
        unsafe {
            mpu.ctrl
                .modify(|r| r | MPU_DEFAULT_MMAP_FOR_PRIVILEGED | MPU_ENABLE);

            scb.shcsr.modify(|r| r | MEMFAULTENA);

            // Ensure MPU settings take effect
            cortex_m::asm::dsb();
            cortex_m::asm::isb();
        }
    }

    // Test write and read
    let ram_slice = unsafe {
        // Get 16-bit words
        let ram_ptr = region_base_address as *mut u16;

        // Convert raw pointer to slice
        let ram_slice = core::slice::from_raw_parts_mut(ram_ptr, size);

        // Return a 4-word slice
        let size = core::mem::size_of::<u16>() * 4usize;
        let mut chunks = ram_slice.chunks_exact_mut(size);
        chunks.next().unwrap()
    };

    // ----------------------------------------------------------
    // Use memory in SDRAM

    ram_slice[0] = 1u16;
    ram_slice[1] = 2;
    ram_slice[2] = 3;
    ram_slice[3] = 4;

    assert_eq!(ram_slice[0], 1);
    assert_eq!(ram_slice[3], 4);

    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().to_Hz();

    loop {
        led_user.on();
        asm::delay(one_second);
        led_user.off();
        asm::delay(one_second);
    }
}
