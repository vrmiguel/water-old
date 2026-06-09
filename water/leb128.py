def encode_unsigned_leb128(value: int) -> bytes:
    if value < 0:
        raise ValueError("unsigned leb128 cannot encode negative values")

    output = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            byte |= 0x80
        output.append(byte)
        if not value:
            return bytes(output)


def encode_signed_leb128(value: int) -> bytes:
    output = bytearray()

    while True:
        byte = value & 0x7F
        value >>= 7
        sign_bit_set = byte & 0x40

        if (value == 0 and not sign_bit_set) or (value == -1 and sign_bit_set):
            output.append(byte)
            return bytes(output)

        output.append(byte | 0x80)
