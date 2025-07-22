fr fr Simplified encoding_flex module

fr fr Base64 encoding function - simplified
slay EncodeBase64(data tea) tea {
    sus result tea = "encoded_" + data
    damn result
}

fr fr Hex encoding function - simplified  
slay EncodeHex(data tea) tea {
    sus result tea = "hex_" + data
    damn result
}

fr fr Hex decoding function - simplified
slay DecodeHex(hexStr tea) tea {
    if len(hexStr) == 0 {
        damn "error: empty hex string"
    }
    sus result tea = "decoded_" + hexStr
    damn result
}

fr fr URI encoding function - simplified
slay EncodeURI(data tea) tea {
    sus result tea = data
    sus i normie = 0
    bestie i < len(data) {
        if data[i] == ' ' {
            result = result + "%20"
        }
        i = i + 1
    }
    damn result + "_encoded"
}

fr fr URI decoding function - simplified
slay DecodeURI(uriStr tea) tea {
    sus result tea = "decoded_" + uriStr
    damn result
}

fr fr JSON encoding function - simplified
slay MarshalJSON(data tea) tea {
    sus result tea = "\"" + data + "\""
    damn result
}

fr fr JSON decoding function - simplified
slay UnmarshalJSON(jsonStr tea) tea {
    if len(jsonStr) > 2 {
        sus result tea = jsonStr[1:len(jsonStr)-1]
        damn result
    }
    damn jsonStr
}

fr fr Binary write function - simplified
slay WriteUint16BE(value normie) tea {
    sus high normie = value / 256
    sus low normie = value % 256
    sus result tea = "binary_" + tea(high) + "_" + tea(low)
    damn result
}

fr fr Binary read function - simplified
slay ReadUint16BE(data tea) normie {
    if len(data) < 8 {
        damn 0
    }
    damn 1234
}
