fr fr LZMA COMPRESSION - Complete Implementation
fr fr Pure CURSED implementation of LZMA compression algorithm
fr fr Based on 7z SDK with range coding and optimal parsing

yeet "mathz"
yeet "stringz"
yeet "vibez"

fr fr ===== LZMA CONSTANTS =====

sus LZMA_DICT_SIZE_MIN drip = 4096        fr fr 4KB minimum dictionary
sus LZMA_DICT_SIZE_MAX drip = 134217728   fr fr 128MB maximum dictionary
sus LZMA_MATCH_LEN_MIN drip = 2           fr fr Minimum match length
sus LZMA_MATCH_LEN_MAX drip = 273         fr fr Maximum match length
sus LZMA_NUM_REPS drip = 4                fr fr Number of repeat distances
sus LZMA_NUM_STATES drip = 12             fr fr Number of LZMA states
sus LZMA_NUM_POS_BITS_MAX drip = 4        fr fr Maximum position bits
sus LZMA_NUM_LIT_POS_BITS_MAX drip = 4    fr fr Maximum literal position bits
sus LZMA_NUM_LIT_CONTEXT_BITS_MAX drip = 8 fr fr Maximum literal context bits

fr fr Range encoder constants
sus RC_BIT_MODEL_TOTAL_BITS drip = 11
sus RC_BIT_MODEL_TOTAL drip = 2048        fr fr 1 << RC_BIT_MODEL_TOTAL_BITS
sus RC_MOVE_BITS drip = 5
sus RC_TOP_VALUE drip = 16777216          fr fr 1 << 24

fr fr ===== LZMA STRUCTURES =====

squad LzmaState {
    sus state drip
    sus rep0 drip
    sus rep1 drip
    sus rep2 drip
    sus rep3 drip
}

squad RangeEncoder {
    sus range drip
    sus low drip
    sus cache_size drip
    sus cache drip
    sus output tea
    sus position drip
}

squad MatchFinder {
    sus hash_table []drip
    sus son []drip
    sus buffer tea
    sus pos drip
    sus pos_limit drip
    sus stream_pos drip
    sus match_max_len drip
    sus nice_len drip
    sus cut_value drip
    sus btMode lit
    sus numHashBytes drip
    sus historySize drip
    sus fixedHashSize drip
    sus hashSizeSum drip
    sus son_refs []drip
}

squad LzmaEncoder {
    sus match_finder MatchFinder
    sus range_encoder RangeEncoder
    sus state LzmaState
    sus literal_probs []drip
    sus rep_probs []drip
    sus match_probs []drip
    sus pos_slot_probs []drip
    sus align_probs []drip
    sus pos_encoders []drip
    sus len_encoder tea         fr fr LenEncoder structure
    sus rep_len_encoder tea     fr fr RepLenEncoder structure
    sus optimal []tea           fr fr Optimal parsing array
    sus fast_mode lit
    sus dictionary_size drip
    sus lc drip                 fr fr literal context bits
    sus lp drip                 fr fr literal position bits
    sus pb drip                 fr fr position bits
}

fr fr ===== RANGE ENCODER IMPLEMENTATION =====

slay range_encoder_init(encoder RangeEncoder) RangeEncoder {
    encoder.range = 4294967295  fr fr 0xFFFFFFFF
    encoder.low = 0
    encoder.cache_size = 1
    encoder.cache = 0
    encoder.output = ""
    encoder.position = 0
    damn encoder
}

slay range_encoder_encode_bit(encoder RangeEncoder, prob drip, symbol drip) RangeEncoder {
    sus new_bound drip = (encoder.range >> RC_BIT_MODEL_TOTAL_BITS) * prob
    
    ready (symbol == 0) {
        encoder.range = new_bound
        prob = prob + ((RC_BIT_MODEL_TOTAL - prob) >> RC_MOVE_BITS)
    } otherwise {
        encoder.low = encoder.low + new_bound
        encoder.range = encoder.range - new_bound
        prob = prob - (prob >> RC_MOVE_BITS)
    }
    
    ready (encoder.range < RC_TOP_VALUE) {
        encoder.range = encoder.range << 8
        encoder = range_encoder_shift_low(encoder)
    }
    
    damn encoder
}

slay range_encoder_shift_low(encoder RangeEncoder) RangeEncoder {
    sus low_hi drip = encoder.low >> 32
    
    ready (low_hi != 0 || encoder.low < 4278190080) {  fr fr 0xFF000000
        sus temp drip = encoder.cache
        
        bestie (encoder.cache_size > 0) {
            encoder.output = encoder.output + char((temp + low_hi) & 255)
            temp = 255
            encoder.cache_size = encoder.cache_size - 1
        }
        
        encoder.cache = (encoder.low >> 24) & 255
    }
    
    encoder.cache_size = encoder.cache_size + 1
    encoder.low = (encoder.low & 16777215) << 8  fr fr 0xFFFFFF
    
    damn encoder
}

slay range_encoder_flush(encoder RangeEncoder) RangeEncoder {
    bestie (encoder.cache_size > 0) {
        encoder.output = encoder.output + char(encoder.cache)
        encoder.cache_size = encoder.cache_size - 1
    }
    damn encoder
}

fr fr ===== MATCH FINDER IMPLEMENTATION =====

slay match_finder_init(finder MatchFinder, dict_size drip) MatchFinder {
    finder.historySize = dict_size
    finder.nice_len = 32
    finder.cut_value = 16
    finder.btMode = based
    finder.numHashBytes = 4
    finder.fixedHashSize = 66560  fr fr Hash table size
    finder.hashSizeSum = finder.fixedHashSize
    
    fr fr Initialize hash table
    finder.hash_table = allocate_int_array(finder.hashSizeSum)
    finder.son = allocate_int_array(finder.historySize * 2)
    finder.son_refs = allocate_int_array(finder.historySize)
    
    finder.pos = 0
    finder.pos_limit = 0
    finder.stream_pos = 0
    finder.match_max_len = LZMA_MATCH_LEN_MAX
    finder.buffer = ""
    
    damn finder
}

slay match_finder_set_stream(finder MatchFinder, stream tea) MatchFinder {
    finder.buffer = stream
    finder.stream_pos = string_length(stream)
    finder.pos_limit = finder.stream_pos
    damn finder
}

slay match_finder_get_matches(finder MatchFinder, distances []drip) MatchFinder {
    sus pos drip = finder.pos
    sus len_limit drip = mathz.min(finder.match_max_len, finder.pos_limit - pos)
    sus offset drip = 1
    sus max_len drip = 0
    
    ready (len_limit < LZMA_MATCH_LEN_MIN) {
        damn finder
    }
    
    fr fr Binary tree search for matches
    sus hash drip = calculate_hash(finder.buffer, pos, finder.numHashBytes)
    sus hash_pos drip = finder.hash_table[hash]
    finder.hash_table[hash] = pos
    
    sus son_ptr drip = (pos & (finder.historySize - 1)) * 2
    sus len0 drip = 0
    sus len1 drip = 0
    sus cut_value drip = finder.cut_value
    
    bestie (based) {
        sus delta drip = pos - hash_pos
        ready (cut_value == 0 || delta >= finder.historySize) {
            finder.son[son_ptr] = 0
            finder.son[son_ptr + 1] = 0
            break
        }
        
        cut_value = cut_value - 1
        
        sus match_pos drip = (hash_pos & (finder.historySize - 1)) * 2
        sus cur_match drip = mathz.min(len0, len1)
        sus len drip = get_match_length(finder.buffer, pos, hash_pos, len_limit)
        
        ready (len > max_len) {
            max_len = len
            distances[offset] = len
            distances[offset + 1] = delta
            offset = offset + 2
            
            ready (len == len_limit) {
                finder.son[son_ptr] = finder.son[match_pos]
                finder.son[son_ptr + 1] = finder.son[match_pos + 1]
                break
            }
        }
        
        sus c drip = get_buffer_byte(finder.buffer, hash_pos + cur_match)
        sus pos_c drip = get_buffer_byte(finder.buffer, pos + cur_match)
        
        ready (c < pos_c) {
            finder.son[son_ptr + 1] = hash_pos
            son_ptr = match_pos + 1
            hash_pos = finder.son[son_ptr]
            len1 = len
        } otherwise {
            finder.son[son_ptr] = hash_pos
            son_ptr = match_pos
            hash_pos = finder.son[son_ptr]
            len0 = len
        }
    }
    
    finder.pos = finder.pos + 1
    damn finder
}

slay calculate_hash(buffer tea, pos drip, num_bytes drip) drip {
    sus hash drip = 0
    sus i drip = 0
    
    bestie (i < num_bytes && pos + i < string_length(buffer)) {
        sus byte drip = char_to_number(substring(buffer, pos + i, 1))
        hash = (hash * 31 + byte) & 65535  fr fr 16-bit hash
        i = i + 1
    }
    
    damn hash
}

slay get_match_length(buffer tea, pos1 drip, pos2 drip, len_limit drip) drip {
    sus len drip = 0
    
    bestie (len < len_limit && pos1 + len < string_length(buffer) && pos2 + len < string_length(buffer)) {
        sus c1 drip = char_to_number(substring(buffer, pos1 + len, 1))
        sus c2 drip = char_to_number(substring(buffer, pos2 + len, 1))
        
        ready (c1 != c2) {
            break
        }
        
        len = len + 1
    }
    
    damn len
}

slay get_buffer_byte(buffer tea, pos drip) drip {
    ready (pos >= string_length(buffer)) {
        damn 0
    }
    damn char_to_number(substring(buffer, pos, 1))
}

fr fr ===== OPTIMAL PARSING =====

squad Optimal {
    sus state drip
    sus prev1_is_char lit
    sus prev2 drip
    sus pos_prev2 drip
    sus back_prev drip
    sus price drip
    sus pos_prev drip
    sus back_cur drip
    sus backs []drip
}

slay get_optimal_sequence(encoder LzmaEncoder, data tea, pos drip, len drip) []Optimal {
    sus optimal_array []Optimal = allocate_optimal_array(4096)
    sus len_end drip = mathz.min(len, 4095)
    
    fr fr Initialize first optimal
    optimal_array[0].state = encoder.state.state
    optimal_array[0].price = 0
    optimal_array[0].pos_prev = -1
    optimal_array[0].back_cur = -1
    
    sus cur drip = 0
    bestie (cur <= len_end) {
        sus cur_optimal Optimal = optimal_array[cur]
        sus cur_price drip = cur_optimal.price
        
        ready (cur_price == 999999999) {  fr fr Invalid state
            cur = cur + 1
            continue
        }
        
        fr fr Try literal
        ready (pos + cur < string_length(data)) {
            sus literal_price drip = get_literal_price(encoder, data, pos + cur, cur_optimal.state)
            sus next_pos drip = cur + 1
            
            ready (next_pos <= len_end) {
                sus new_price drip = cur_price + literal_price
                ready (optimal_array[next_pos].price > new_price) {
                    optimal_array[next_pos].price = new_price
                    optimal_array[next_pos].pos_prev = cur
                    optimal_array[next_pos].back_cur = -1
                    optimal_array[next_pos].prev1_is_char = based
                    optimal_array[next_pos].state = get_literal_next_state(cur_optimal.state)
                }
            }
        }
        
        fr fr Try matches
        sus distances []drip = allocate_int_array(256)
        encoder.match_finder = match_finder_get_matches(encoder.match_finder, distances)
        
        sus num_pairs drip = distances[0]
        ready (num_pairs > 0) {
            sus match_price drip = get_match_price(encoder, cur_optimal.state)
            
            sus pair_index drip = 1
            bestie (pair_index < num_pairs) {
                sus match_len drip = distances[pair_index]
                sus match_dist drip = distances[pair_index + 1]
                
                sus len_price drip = get_len_price(encoder, match_len, cur_optimal.state)
                sus dist_price drip = get_distance_price(encoder, match_dist)
                sus total_match_price drip = match_price + len_price + dist_price
                
                sus next_pos drip = cur + match_len
                ready (next_pos <= len_end) {
                    sus new_price drip = cur_price + total_match_price
                    ready (optimal_array[next_pos].price > new_price) {
                        optimal_array[next_pos].price = new_price
                        optimal_array[next_pos].pos_prev = cur
                        optimal_array[next_pos].back_cur = match_dist
                        optimal_array[next_pos].prev1_is_char = cringe
                        optimal_array[next_pos].state = get_match_next_state(cur_optimal.state)
                    }
                }
                
                pair_index = pair_index + 2
            }
        }
        
        cur = cur + 1
    }
    
    damn optimal_array
}

fr fr ===== PRICE CALCULATION =====

slay get_literal_price(encoder LzmaEncoder, data tea, pos drip, state drip) drip {
    sus literal_state drip = ((pos & ((1 << encoder.lp) - 1)) << encoder.lc)
    ready (pos > 0) {
        sus prev_byte drip = char_to_number(substring(data, pos - 1, 1))
        literal_state = literal_state + (prev_byte >> (8 - encoder.lc))
    }
    
    sus symbol drip = char_to_number(substring(data, pos, 1))
    sus price drip = 0
    
    ready (is_char_state(state)) {
        fr fr Simple literal encoding
        sus bit drip = 7
        bestie (bit >= 0) {
            sus bit_value drip = (symbol >> bit) & 1
            price = price + get_bit_price(encoder.literal_probs[literal_state], bit_value)
            literal_state = (literal_state << 1) | bit_value
            bit = bit - 1
        }
    } otherwise {
        fr fr Matched literal encoding
        sus match_byte drip = get_rep_byte(encoder, 0, pos)
        sus offset drip = 256
        
        sus bit drip = 7
        bestie (bit >= 0) {
            sus match_bit drip = (match_byte >> bit) & 1
            sus bit_value drip = (symbol >> bit) & 1
            
            price = price + get_bit_price(encoder.literal_probs[literal_state + offset + match_bit], bit_value)
            
            ready (match_bit != bit_value) {
                break
            }
            
            literal_state = (literal_state << 1) | bit_value
            offset = offset & (0 - match_bit)
            bit = bit - 1
        }
        
        bestie (bit >= 0) {
            sus bit_value drip = (symbol >> bit) & 1
            price = price + get_bit_price(encoder.literal_probs[literal_state], bit_value)
            literal_state = (literal_state << 1) | bit_value
            bit = bit - 1
        }
    }
    
    damn price
}

slay get_match_price(encoder LzmaEncoder, state drip) drip {
    damn get_bit_price(encoder.match_probs[state], 1)
}

slay get_len_price(encoder LzmaEncoder, len drip, state drip) drip {
    sus len_state drip = mathz.min(len - LZMA_MATCH_LEN_MIN, 15)
    sus price drip = 0
    
    ready (len_state < 8) {
        price = get_bit_price(encoder.len_encoder, 0)
        price = price + get_bits_price(encoder.len_encoder, len_state, 3)
    } otherwise {
        price = get_bit_price(encoder.len_encoder, 1)
        price = price + get_bits_price(encoder.len_encoder, len_state - 8, 3)
    }
    
    damn price
}

slay get_distance_price(encoder LzmaEncoder, distance drip) drip {
    sus pos_slot drip = get_pos_slot(distance)
    sus price drip = get_bits_price(encoder.pos_slot_probs, pos_slot, 6)
    
    ready (pos_slot >= 4) {
        sus footer_bits drip = (pos_slot >> 1) - 1
        sus base drip = ((2 | (pos_slot & 1)) << footer_bits)
        sus dist_reduced drip = distance - base
        
        ready (pos_slot < 14) {
            price = price + get_reverse_bits_price(encoder.pos_encoders, dist_reduced, footer_bits)
        } otherwise {
            price = price + get_direct_bits_price(dist_reduced >> 4, footer_bits - 4)
            price = price + get_reverse_bits_price(encoder.align_probs, dist_reduced & 15, 4)
        }
    }
    
    damn price
}

slay get_bit_price(prob drip, symbol drip) drip {
    ready (symbol == 0) {
        damn prob >> RC_MOVE_BITS
    } otherwise {
        damn (RC_BIT_MODEL_TOTAL - prob) >> RC_MOVE_BITS
    }
}

slay get_bits_price(probs []drip, symbol drip, bit_count drip) drip {
    sus price drip = 0
    sus bit drip = bit_count - 1
    
    bestie (bit >= 0) {
        sus bit_value drip = (symbol >> bit) & 1
        price = price + get_bit_price(probs[1], bit_value)
        bit = bit - 1
    }
    
    damn price
}

fr fr ===== STATE TRANSITIONS =====

slay get_literal_next_state(state drip) drip {
    ready (state < 4) { damn 0 }
    ready (state < 7) { damn state - 3 }
    ready (state < 10) { damn state - 6 }
    damn state - 9
}

slay get_match_next_state(state drip) drip {
    ready (state < 7) { damn 7 }
    damn 10
}

slay is_char_state(state drip) lit {
    damn state < 7
}

slay get_pos_slot(distance drip) drip {
    ready (distance < 4) { damn distance }
    ready (distance < 128) { damn 4 + ((distance - 4) >> 1) }
    ready (distance < 2048) { damn 10 + ((distance - 128) >> 4) }
    ready (distance < 32768) { damn 16 + ((distance - 2048) >> 8) }
    damn 22 + ((distance - 32768) >> 12)
}

fr fr ===== ENCODING FUNCTIONS =====

slay encode_literal(encoder LzmaEncoder, symbol drip, match_byte drip, state drip) LzmaEncoder {
    sus literal_state drip = get_literal_state(encoder, encoder.match_finder.pos)
    
    ready (is_char_state(state)) {
        sus context drip = 1
        sus i drip = 7
        bestie (i >= 0) {
            sus bit drip = (symbol >> i) & 1
            encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder, 
                encoder.literal_probs[literal_state + context], bit)
            context = (context << 1) | bit
            i = i - 1
        }
    } otherwise {
        sus context drip = 1
        sus i drip = 7
        bestie (i >= 0) {
            sus match_bit drip = (match_byte >> i) & 1
            sus bit drip = (symbol >> i) & 1
            sus same drip = ready (match_bit == bit) { 1 } otherwise { 0 }
            
            encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder,
                encoder.literal_probs[literal_state + ((1 + match_bit) << 8) + context], bit)
            
            context = (context << 1) | bit
            ready (!same) {
                i = i - 1
                break
            }
            i = i - 1
        }
        
        bestie (i >= 0) {
            sus bit drip = (symbol >> i) & 1
            encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder,
                encoder.literal_probs[literal_state + context], bit)
            context = (context << 1) | bit
            i = i - 1
        }
    }
    
    damn encoder
}

slay encode_match(encoder LzmaEncoder, len drip, distance drip, state drip) LzmaEncoder {
    fr fr Encode match bit
    encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder,
        encoder.match_probs[state], 1)
    
    fr fr Encode length
    encoder = encode_length(encoder, len - LZMA_MATCH_LEN_MIN)
    
    fr fr Encode distance
    sus pos_slot drip = get_pos_slot(distance)
    encoder = encode_pos_slot(encoder, pos_slot)
    
    ready (pos_slot >= 4) {
        sus footer_bits drip = (pos_slot >> 1) - 1
        sus base drip = ((2 | (pos_slot & 1)) << footer_bits)
        sus pos_reduced drip = distance - base
        
        ready (pos_slot < 14) {
            encoder = encode_reverse_bits(encoder, encoder.pos_encoders, pos_reduced, footer_bits)
        } otherwise {
            encoder = encode_direct_bits(encoder, pos_reduced >> 4, footer_bits - 4)
            encoder = encode_reverse_bits(encoder, encoder.align_probs, pos_reduced & 15, 4)
        }
    }
    
    damn encoder
}

fr fr ===== MAIN COMPRESSION FUNCTION =====

slay lzma_compress_advanced(data tea, dict_size drip, lc drip, lp drip, pb drip) tea {
    vibez.spill("LZMA: Advanced compression - data size: " + number_to_string(string_length(data)))
    
    fr fr Initialize encoder
    sus encoder LzmaEncoder = create_lzma_encoder(dict_size, lc, lp, pb)
    encoder.match_finder = match_finder_init(encoder.match_finder, dict_size)
    encoder.match_finder = match_finder_set_stream(encoder.match_finder, data)
    encoder.range_encoder = range_encoder_init(encoder.range_encoder)
    
    fr fr Compress data using optimal parsing
    sus data_len drip = string_length(data)
    sus pos drip = 0
    
    bestie (pos < data_len) {
        sus remaining drip = data_len - pos
        sus len drip = mathz.min(remaining, 4096)
        
        fr fr Get optimal sequence for current window
        sus optimal_sequence []Optimal = get_optimal_sequence(encoder, data, pos, len)
        
        fr fr Encode optimal sequence
        sus opt_pos drip = len
        bestie (opt_pos > 0) {
            sus opt Optimal = optimal_sequence[opt_pos]
            ready (opt.back_cur >= 0) {
                fr fr Match
                sus match_len drip = opt_pos - opt.pos_prev
                encoder = encode_match(encoder, match_len, opt.back_cur, encoder.state.state)
                encoder.state.state = get_match_next_state(encoder.state.state)
                update_reps(encoder, opt.back_cur)
            } otherwise {
                fr fr Literal
                sus literal_pos drip = pos + opt.pos_prev
                sus symbol drip = char_to_number(substring(data, literal_pos, 1))
                sus match_byte drip = get_rep_byte(encoder, 0, literal_pos)
                encoder = encode_literal(encoder, symbol, match_byte, encoder.state.state)
                encoder.state.state = get_literal_next_state(encoder.state.state)
            }
            opt_pos = opt.pos_prev
        }
        
        pos = pos + len
    }
    
    fr fr Flush range encoder
    encoder.range_encoder = range_encoder_flush(encoder.range_encoder)
    
    fr fr Create LZMA header
    sus header tea = create_lzma_header(dict_size, lc, lp, pb, string_length(data))
    
    vibez.spill("LZMA: Compressed " + number_to_string(string_length(data)) + " -> " + 
               number_to_string(string_length(encoder.range_encoder.output)) + " bytes")
    
    damn header + encoder.range_encoder.output
}

fr fr ===== HELPER FUNCTIONS =====

slay create_lzma_encoder(dict_size drip, lc drip, lp drip, pb drip) LzmaEncoder {
    sus encoder LzmaEncoder
    encoder.dictionary_size = dict_size
    encoder.lc = lc
    encoder.lp = lp
    encoder.pb = pb
    encoder.fast_mode = cringe
    
    fr fr Initialize probability arrays
    encoder.literal_probs = allocate_int_array(768 * (1 << (lc + lp)))
    encoder.match_probs = allocate_int_array(192)
    encoder.rep_probs = allocate_int_array(16)
    encoder.pos_slot_probs = allocate_int_array(64 * 16)
    encoder.align_probs = allocate_int_array(16)
    encoder.pos_encoders = allocate_int_array(114)
    
    fr fr Initialize all probabilities to middle value
    init_bit_models(encoder.literal_probs, 768 * (1 << (lc + lp)))
    init_bit_models(encoder.match_probs, 192)
    init_bit_models(encoder.rep_probs, 16)
    init_bit_models(encoder.pos_slot_probs, 64 * 16)
    init_bit_models(encoder.align_probs, 16)
    init_bit_models(encoder.pos_encoders, 114)
    
    fr fr Initialize state
    encoder.state.state = 0
    encoder.state.rep0 = 0
    encoder.state.rep1 = 0
    encoder.state.rep2 = 0
    encoder.state.rep3 = 0
    
    damn encoder
}

slay init_bit_models(probs []drip, count drip) {
    sus i drip = 0
    bestie (i < count) {
        probs[i] = RC_BIT_MODEL_TOTAL / 2  fr fr Middle probability
        i = i + 1
    }
}

slay create_lzma_header(dict_size drip, lc drip, lp drip, pb drip, uncompressed_size drip) tea {
    sus properties drip = (pb * 5 + lp) * 9 + lc
    sus header tea = char(properties & 255)
    
    fr fr Encode dictionary size (4 bytes little endian)
    header = header + char(dict_size & 255)
    header = header + char((dict_size >> 8) & 255)
    header = header + char((dict_size >> 16) & 255)
    header = header + char((dict_size >> 24) & 255)
    
    fr fr Encode uncompressed size (8 bytes little endian)
    header = header + char(uncompressed_size & 255)
    header = header + char((uncompressed_size >> 8) & 255)
    header = header + char((uncompressed_size >> 16) & 255)
    header = header + char((uncompressed_size >> 24) & 255)
    header = header + char(0) + char(0) + char(0) + char(0)  fr fr High 32 bits
    
    damn header
}

slay allocate_int_array(size drip) []drip {
    sus array []drip = []
    sus i drip = 0
    bestie (i < size) {
        array = array + [0]
        i = i + 1
    }
    damn array
}

slay allocate_optimal_array(size drip) []Optimal {
    sus array []Optimal = []
    sus i drip = 0
    bestie (i < size) {
        sus opt Optimal
        opt.price = 999999999  fr fr Infinity
        opt.pos_prev = -1
        opt.back_cur = -1
        opt.state = 0
        opt.prev1_is_char = cringe
        array = array + [opt]
        i = i + 1
    }
    damn array
}

slay get_literal_state(encoder LzmaEncoder, pos drip) drip {
    damn ((pos & ((1 << encoder.lp) - 1)) << encoder.lc)
}

slay get_rep_byte(encoder LzmaEncoder, rep drip, pos drip) drip {
    sus rep_distance drip
    ready (rep == 0) { rep_distance = encoder.state.rep0 }
    ready (rep == 1) { rep_distance = encoder.state.rep1 }
    ready (rep == 2) { rep_distance = encoder.state.rep2 }
    ready (rep == 3) { rep_distance = encoder.state.rep3 }
    
    sus rep_pos drip = pos - rep_distance - 1
    ready (rep_pos < 0) { damn 0 }
    ready (rep_pos >= string_length(encoder.match_finder.buffer)) { damn 0 }
    
    damn char_to_number(substring(encoder.match_finder.buffer, rep_pos, 1))
}

slay update_reps(encoder LzmaEncoder, distance drip) {
    ready (distance != encoder.state.rep0) {
        ready (distance != encoder.state.rep1) {
            ready (distance != encoder.state.rep2) {
                encoder.state.rep3 = encoder.state.rep2
            }
            encoder.state.rep2 = encoder.state.rep1
        }
        encoder.state.rep1 = encoder.state.rep0
    }
    encoder.state.rep0 = distance
}

fr fr ===== ADDITIONAL ENCODING FUNCTIONS =====

slay encode_length(encoder LzmaEncoder, len drip) LzmaEncoder {
    ready (len < 8) {
        encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder, encoder.len_encoder, 0)
        encoder = encode_tree_bits(encoder, encoder.len_encoder, len, 3)
    } otherwise {
        encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder, encoder.len_encoder, 1)
        encoder = encode_tree_bits(encoder, encoder.len_encoder, len - 8, 3)
    }
    damn encoder
}

slay encode_pos_slot(encoder LzmaEncoder, pos_slot drip) LzmaEncoder {
    encoder = encode_tree_bits(encoder, encoder.pos_slot_probs, pos_slot, 6)
    damn encoder
}

slay encode_tree_bits(encoder LzmaEncoder, probs []drip, symbol drip, bit_count drip) LzmaEncoder {
    sus model_index drip = 1
    sus bit drip = bit_count - 1
    
    bestie (bit >= 0) {
        sus bit_value drip = (symbol >> bit) & 1
        encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder, 
            probs[model_index], bit_value)
        model_index = (model_index << 1) | bit_value
        bit = bit - 1
    }
    
    damn encoder
}

slay encode_reverse_bits(encoder LzmaEncoder, probs []drip, symbol drip, bit_count drip) LzmaEncoder {
    sus model_index drip = 1
    sus i drip = 0
    
    bestie (i < bit_count) {
        sus bit drip = (symbol >> i) & 1
        encoder.range_encoder = range_encoder_encode_bit(encoder.range_encoder,
            probs[model_index], bit)
        model_index = (model_index << 1) | bit
        i = i + 1
    }
    
    damn encoder
}

slay encode_direct_bits(encoder LzmaEncoder, value drip, bit_count drip) LzmaEncoder {
    sus i drip = bit_count - 1
    bestie (i >= 0) {
        encoder.range_encoder.range = encoder.range_encoder.range >> 1
        ready ((value >> i) & 1) {
            encoder.range_encoder.low = encoder.range_encoder.low + encoder.range_encoder.range
        }
        ready (encoder.range_encoder.range < RC_TOP_VALUE) {
            encoder.range_encoder.range = encoder.range_encoder.range << 8
            encoder.range_encoder = range_encoder_shift_low(encoder.range_encoder)
        }
        i = i - 1
    }
    damn encoder
}

slay get_reverse_bits_price(probs []drip, symbol drip, bit_count drip) drip {
    sus price drip = 0
    sus model_index drip = 1
    sus i drip = 0
    
    bestie (i < bit_count) {
        sus bit drip = (symbol >> i) & 1
        price = price + get_bit_price(probs[model_index], bit)
        model_index = (model_index << 1) | bit
        i = i + 1
    }
    
    damn price
}

slay get_direct_bits_price(value drip, bit_count drip) drip {
    damn bit_count * 128  fr fr Fixed cost per direct bit
}

slay number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num < 0) { damn "-" + number_to_string(-num) }
    
    sus result tea = ""
    bestie (num > 0) {
        sus digit drip = num % 10
        result = char(48 + digit) + result
        num = num / 10
    }
    damn result
}
