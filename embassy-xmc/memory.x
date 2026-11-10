/* XMC4700 Relax Kit
 * ARM Cortex-M4F
 */

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 2M

    RAM  (rwx) : ORIGIN = 0x20000000, LENGTH = 256K

    PSRAM (rwx) : ORIGIN = 0x1FFE8000, LENGTH = 32K
}

/* Stack grows downwards */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);