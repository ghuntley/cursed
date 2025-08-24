# Regular Expression Public API
# High-level interface for regex operations

yeet "regexz/regex_engine"

# Main regex compilation function
slay regex_new(pattern tea) yikes<RegexEngine> {
    sus options RegexOptions = create_default_options()
    damn get_cached_engine(pattern, options)
}

slay regex_new_with_options(pattern tea, options RegexOptions) yikes<RegexEngine> {
    damn get_cached_engine(pattern, options)
}

# Pattern matching functions
slay regex_match(engine *RegexEngine, text tea) yikes<MatchResult> {
    damn match_regex(engine, text, 0)
}

slay regex_match_at(engine *RegexEngine, text tea, start_pos drip) yikes<MatchResult> {
    damn match_regex(engine, text, start_pos)
}

slay regex_find_all(engine *RegexEngine, text tea) yikes<[]MatchResult> {
    sus matches []MatchResult = create_array()
    sus position drip = 0
    
    bestie (position < text.len()) {
        sus result MatchResult = match_regex(engine, text, position) fam {
            when error -> yikes error
        }
        
        ready (!result.matched) {
            break
        }
        
        matches.push(result)
        position = result.end_pos
        
        # Prevent infinite loop on empty matches
        ready (result.start_pos == result.end_pos) {
            position += 1
        }
    }
    
    damn matches
}

# Replacement functions
slay regex_replace(engine *RegexEngine, text tea, replacement tea) yikes<tea> {
    sus matches []MatchResult = regex_find_all(engine, text) fam {
        when error -> yikes error
    }
    
    ready (matches.len() == 0) {
        damn text
    }
    
    sus result tea = ""
    sus last_pos drip = 0
    
    bestie (match in matches) {
        # Add text before match
        result += text.substring(last_pos, match.start_pos)
        
        # Add replacement (with group substitution)
        result += expand_replacement(replacement, match)
        
        last_pos = match.end_pos
    }
    
    # Add remaining text
    result += text.substring(last_pos, text.len())
    
    damn result
}

slay regex_replace_func(engine *RegexEngine, text tea, replacer slay(MatchResult) tea) yikes<tea> {
    sus matches []MatchResult = regex_find_all(engine, text) fam {
        when error -> yikes error
    }
    
    ready (matches.len() == 0) {
        damn text
    }
    
    sus result tea = ""
    sus last_pos drip = 0
    
    bestie (match in matches) {
        result += text.substring(last_pos, match.start_pos)
        result += replacer(match)
        last_pos = match.end_pos
    }
    
    result += text.substring(last_pos, text.len())
    damn result
}

# String splitting
slay regex_split(engine *RegexEngine, text tea) yikes<[]tea> {
    sus matches []MatchResult = regex_find_all(engine, text) fam {
        when error -> yikes error
    }
    
    ready (matches.len() == 0) {
        damn [text]
    }
    
    sus parts []tea = create_array()
    sus last_pos drip = 0
    
    bestie (match in matches) {
        ready (match.start_pos > last_pos) {
            parts.push(text.substring(last_pos, match.start_pos))
        }
        last_pos = match.end_pos
    }
    
    # Add final part
    ready (last_pos < text.len()) {
        parts.push(text.substring(last_pos, text.len()))
    }
    
    damn parts
}

# Validation functions
slay regex_is_valid(pattern tea) lit {
    sus options RegexOptions = create_default_options()
    sus engine RegexEngine = compile_regex(pattern, options) fam {
        when _ -> damn nah
    }
    damn based
}

slay regex_test(pattern tea, text tea) yikes<lit> {
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> yikes error
    }
    
    sus result MatchResult = regex_match(&engine, text) fam {
        when error -> yikes error
    }
    
    damn result.matched
}

# Named group extraction
slay get_named_group(result MatchResult, name tea) yikes<tea> {
    ready (!result.named_groups.has(name)) {
        yikes "group '" + name + "' not found"
    }
    
    damn result.named_groups[name].value
}

slay get_all_named_groups(result MatchResult) map<tea, tea> {
    sus groups map<tea, tea> = create_map()
    
    bestie ((name, group) in result.named_groups.entries()) {
        groups[name] = group.value
    }
    
    damn groups
}

# Advanced pattern building
slay regex_escape(text tea) tea {
    sus escaped tea = ""
    
    bestie (ch in text.chars()) {
        ready (is_special_char(ch)) {
            escaped += "\\"
        }
        escaped += ch.to_string()
    }
    
    damn escaped
}

slay is_special_char(ch drip) lit {
    sus special_chars tea = ".*+?^${}()|[]\\"
    damn special_chars.contains(ch.to_string())
}

# Replacement string expansion
slay expand_replacement(replacement tea, match MatchResult) tea {
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < replacement.len()) {
        ready (replacement.char_at(i) == "\\".char_at(0)) {
            ready (i + 1 < replacement.len()) {
                sus next_char drip = replacement.char_at(i + 1)
                
                sick (next_char.to_string()) {
                    when "&" -> result += match.full_match
                    when "0" -> result += match.full_match
                    when "1" -> ready (match.groups.len() > 0) { result += match.groups[0].value }
                    when "2" -> ready (match.groups.len() > 1) { result += match.groups[1].value }
                    when "3" -> ready (match.groups.len() > 2) { result += match.groups[2].value }
                    when "4" -> ready (match.groups.len() > 3) { result += match.groups[3].value }
                    when "5" -> ready (match.groups.len() > 4) { result += match.groups[4].value }
                    when "6" -> ready (match.groups.len() > 5) { result += match.groups[5].value }
                    when "7" -> ready (match.groups.len() > 6) { result += match.groups[6].value }
                    when "8" -> ready (match.groups.len() > 7) { result += match.groups[7].value }
                    when "9" -> ready (match.groups.len() > 8) { result += match.groups[8].value }
                    when "\\" -> result += "\\"
                    when _ -> result += next_char.to_string()
                }
                i += 2
            } otherwise {
                result += replacement.char_at(i).to_string()
                i += 1
            }
        } otherwise {
            result += replacement.char_at(i).to_string()
            i += 1
        }
    }
    
    damn result
}

# Pattern analysis
slay analyze_pattern(pattern tea) yikes<PatternAnalysis> {
    sus engine RegexEngine = compile_regex(pattern, create_default_options()) fam {
        when error -> yikes error
    }
    
    sus analysis PatternAnalysis = {
        pattern: pattern,
        has_groups: engine.compiled_nfa.states.len() > 0,
        has_lookaround: contains_lookaround(&engine.compiled_nfa),
        estimated_complexity: estimate_complexity(&engine.compiled_nfa),
        unicode_aware: engine.unicode_support
    }
    
    damn analysis
}

squad PatternAnalysis {
    sus pattern tea
    sus has_groups lit
    sus has_lookaround lit
    sus estimated_complexity drip  # 1=linear, 2=polynomial, 3=exponential
    sus unicode_aware lit
}

# Performance monitoring
squad RegexStats {
    sus pattern tea
    sus match_count drip
    sus total_match_time drip  # microseconds
    sus cache_hits drip
    sus cache_misses drip
}

sus global_regex_stats map<tea, RegexStats> = create_map()

slay get_regex_stats(pattern tea) RegexStats {
    ready (global_regex_stats.has(pattern)) {
        damn global_regex_stats[pattern]
    }
    
    damn RegexStats{
        pattern: pattern,
        match_count: 0,
        total_match_time: 0,
        cache_hits: 0,
        cache_misses: 0
    }
}

slay reset_regex_stats() drip {
    global_regex_stats = create_map()
}
