yeet "asn1_mood"

# ASN.1 Module Demonstration
vibez.spill("ASN.1 Module Demo - Pure CURSED Implementation")

# Create various ASN.1 objects
sus int_obj ASN1Object = asn1_int_new(42)
sus str_obj ASN1Object = asn1_string_new("Hello ASN.1")
sus seq_obj ASN1Object = asn1_sequence_new()
sus oid_obj ASN1Object = asn1_oid_new("1.2.840.113549.1.1.1")
sus time_obj ASN1Object = asn1_time_new("20231207120000Z")

vibez.spill("Created ASN.1 objects:")
vibez.spill("- Integer object (tag 2)")
vibez.spill("- String object (tag 4)")
vibez.spill("- Sequence object (tag 16)")
vibez.spill("- OID object (tag 6)")
vibez.spill("- Time object (tag 23)")

# Encode objects
sus encoded_int tea = asn1_encode_der(int_obj)
sus encoded_str tea = asn1_encode_der(str_obj)
sus encoded_seq tea = asn1_encode_der(seq_obj)
sus encoded_oid tea = asn1_encode_der(oid_obj)
sus encoded_time tea = asn1_encode_der(time_obj)

vibez.spill("Encoded all objects to DER format")

# Decode objects
sus decoded_int ASN1Object = asn1_parse_der(encoded_int)
sus decoded_str ASN1Object = asn1_parse_der(encoded_str)
sus decoded_seq ASN1Object = asn1_parse_der(encoded_seq)
sus decoded_oid ASN1Object = asn1_parse_der(encoded_oid)
sus decoded_time ASN1Object = asn1_parse_der(encoded_time)

vibez.spill("Decoded all objects from DER format")

# Verify tag types
vibez.spill("Verification:")
bestie decoded_int.tag.tag_number == ASN1_INTEGER {
    vibez.spill("✓ Integer object decoded correctly")
}

bestie decoded_str.tag.tag_number == ASN1_OCTET_STRING {
    vibez.spill("✓ String object decoded correctly")
}

bestie decoded_seq.tag.tag_number == ASN1_SEQUENCE {
    vibez.spill("✓ Sequence object decoded correctly")
}

bestie decoded_oid.tag.tag_number == ASN1_OBJECT_IDENTIFIER {
    vibez.spill("✓ OID object decoded correctly")
}

bestie decoded_time.tag.tag_number == ASN1_UTC_TIME {
    vibez.spill("✓ Time object decoded correctly")
}

vibez.spill("ASN.1 demo completed successfully!")
