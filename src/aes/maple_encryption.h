#ifndef MAPLEENCRYPTION_H
#define MAPLEENCRYPTION_H
#if defined(__cplusplus)
extern "C"
{
#endif
    void ffi_decrypt(unsigned char *buffer, unsigned char *iv, unsigned short size);
    void ffi_encrypt(unsigned char *buffer, unsigned char *iv, unsigned short size);
    void ffi_create_packet_header(unsigned char *buffer, unsigned char *iv, unsigned short size);
    unsigned short ffi_get_packet_length(unsigned char *buffer);

#if defined(__cplusplus)
}
#endif
#endif
