
/* Table of CRCs of all 8-bit messages. */
unsigned long crc_table[256];

/* Flag: has the table been computed? Initially false. */
int crc_table_computed;

/* Make the table for a fast CRC. */
void make_crc_table(void);

/* Update a running CRC with the bytes buf[0..len-1]--the CRC
    should be initialized to all 1's, and the transmitted value
    is the 1's complement of the final running CRC (see the
    crc() routine below)). */

unsigned long update_crc(unsigned long crc, unsigned char *buf,int len, int chk);

/* Return the CRC of the bytes buf[0..len-1]. */
unsigned long crc(unsigned char *buf, int len, int chk);