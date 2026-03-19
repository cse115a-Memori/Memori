use std::{
    fs::{self, File},
    path::Path,
};

use nanoid::nanoid;
use rand::Rng;

const ID_ALPHABET: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let pair_code = nanoid!(4, &ID_ALPHABET);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let ble_id_dest = Path::new(&out_dir).join("device_identity.bin");

    let mut ble_address: [u8; 6] = Default::default();
    let mut rng = rand::rng();
    rng.fill_bytes(&mut ble_address);

    // the top two bits need to be high
    // for a static random address to show up in bluetooth
    ble_address[0] |= 0b1100_0000;

    fs::write(&ble_id_dest, ble_address).expect("should write the id.");

    println!("cargo:rustc-env=DEVICE_ID={}", pair_code);
    println!("cargo:rerun-if-env-changed=RANDOM_SEED");

    linker_be_nice();
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                what if what.starts_with("_defmt_") => {
                    eprintln!();
                    eprintln!(
                        "💡 `defmt` not found - make sure `defmt.x` is added as a linker script and you have included `use defmt_rtt as _;`"
                    );
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the linker script `linkall.x` missing?");
                    eprintln!();
                }
                what if what.starts_with("esp_rtos_") => {
                    eprintln!();
                    eprintln!(
                        "💡 `esp-radio` has no scheduler enabled. Make sure you have initialized `esp-rtos` or provided an external scheduler."
                    );
                    eprintln!();
                }
                "embedded_test_linker_file_not_added_to_rustflags" => {
                    eprintln!();
                    eprintln!(
                        "💡 `embedded-test` not found - make sure `embedded-test.x` is added as a linker script for tests"
                    );
                    eprintln!();
                }
                "free"
                | "malloc"
                | "calloc"
                | "get_free_internal_heap_size"
                | "malloc_internal"
                | "realloc_internal"
                | "calloc_internal"
                | "free_internal" => {
                    eprintln!();
                    eprintln!(
                        "💡 Did you forget the `esp-alloc` dependency or didn't enable the `compat` feature on it?"
                    );
                    eprintln!();
                }
                _ => (),
            },
            // we don't have anything helpful for "missing-lib" yet
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}
