use cortex_m::asm;

#[derive(Clone, Copy)]
#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

// Gemeinsamer Handler für noch nicht implementierte Interrupts
unsafe extern "C" fn default_handler() {
    loop {
        asm::wfi();
    }
}

// Interrupt-Tabelle für den XMC4700
const fn make_interrupts() -> [Vector; 112] {
    let mut vectors = [
        Vector {
            handler: default_handler,
        };
        112
    ];

    // Reservierte Interrupts laut Infineon-Referenzhandbuch
    vectors[9] = Vector { reserved: 0 };
    vectors[10] = Vector { reserved: 0 };
    vectors[11] = Vector { reserved: 0 };
    vectors[13] = Vector { reserved: 0 };

    vectors
}

// Die eigentliche Interrupt-Vektortabelle
//#[used]
//#[unsafe(link_section = ".vector_table.interrupts")]
//#[unsafe(no_mangle)]
//pub static __INTERRUPTS: [Vector; 112] = make_interrupts();