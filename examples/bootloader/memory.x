/* See: https://github.com/electro-smith/libDaisy/blob/master/core/STM32H750IB_sram.lds */

MEMORY
{
	DTCMRAM     (RWX) : ORIGIN = 0x20000000, LENGTH = 128K
	SRAM        (RWX) : ORIGIN = 0x24000000, LENGTH = 512K - 32K
	RAM_D2_DMA  (RWX) : ORIGIN = 0x30000000, LENGTH = 32K
	RAM_D2      (RWX) : ORIGIN = 0x30008000, LENGTH = 256K
	RAM_D3      (RWX) : ORIGIN = 0x38000000, LENGTH = 64K
	BACKUP_SRAM (RWX) : ORIGIN = 0x38800000, LENGTH = 4K
	ITCMRAM     (RWX) : ORIGIN = 0x00000000, LENGTH = 64K
	SDRAM       (RWX) : ORIGIN = 0xc0000000, LENGTH = 64M
	QSPIFLASH   (RX)  : ORIGIN = 0x90040000, LENGTH = 7936K
}

REGION_ALIAS(RAM, DTCMRAM);
REGION_ALIAS("FLASH", SRAM);
