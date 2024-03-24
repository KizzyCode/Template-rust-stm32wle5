/* Memory for the STM32WLE5 */
MEMORY
{
  /* See section 4.3.1 "Flash memory organization" in the reference manual */
  FLASH (RX) : ORIGIN = 0x8000000, LENGTH = 256k - 8k
  DATA (RW) : ORIGIN = 0x803E000, LENGTH = 8k
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 64K
}

/* Memory section config */
SECTIONS {
  /* Word-aligned user-data section */
  .userdata (NOLOAD) :
  {
    . = ALIGN(4);
    *(.userdata)
    . = ALIGN(4);
  } > DATA
}
