fr fr SpillFacts module - Enhanced formatting and printing
fr fr Provides comprehensive string formatting, styled output, and advanced printing capabilities

fr fr Color constants
sus Red tea = "red"
sus Green tea = "green"
sus Yellow tea = "yellow"
sus Blue tea = "blue"
sus Magenta tea = "magenta"
sus Cyan tea = "cyan"
sus White tea = "white"
sus Black tea = "black"

fr fr Style constants
sus Bold tea = "bold"
sus Italic tea = "italic"
sus Underline tea = "underline"
sus Blink tea = "blink"
sus Reverse tea = "reverse"
sus Strike tea = "strike"

fr fr GenZ format constants
sus FormatBasic tea = "basic"
sus FormatVibe tea = "vibe"
sus FormatBussin tea = "bussin"
sus FormatSus tea = "sus"
sus FormatYeet tea = "yeet"
sus FormatNoCapFr tea = "nocapfr"
sus FormatDownBad tea = "downbad"

fr fr Global default GenZ format
sus defaultGenZFormat tea = "basic"

fr fr Basic printing functions
slay Spill(message tea) tea {
    vibez.spill(message)
    damn message
}

slay SpillLine(message tea) tea {
    vibez.spill(message)
    damn message
}

slay SpillFormat(format tea, value tea) tea {
    sus formatted tea = format + " " + value
    vibez.spill(formatted)
    damn formatted
}

fr fr String formatting functions
slay GetFacts(message tea) tea {
    damn message
}

slay GetFactsFormat(format tea, value tea) tea {
    sus formatted tea = format + " " + value
    damn formatted
}

slay GetFactsLine(message tea) tea {
    damn message
}

fr fr Error formatting
slay CapError(format tea, value tea) tea {
    sus errorMsg tea = "Error: " + format + " " + value
    damn errorMsg
}

fr fr Styled output functions
slay SpillColor(color tea, message tea) tea {
    sus colorCodes tea = "{" + color + "}" + message + "{reset}"
    vibez.spill(colorCodes)
    damn colorCodes
}

slay SpillStyle(style tea, message tea) tea {
    sus styleCodes tea = "[" + style + "]" + message + "[reset]"
    vibez.spill(styleCodes)
    damn styleCodes
}

slay ColorFacts(color tea, message tea) tea {
    sus coloredText tea = "{" + color + "}" + message + "{reset}"
    damn coloredText
}

slay StyleFacts(style tea, message tea) tea {
    sus styledText tea = "[" + style + "]" + message + "[reset]"
    damn styledText
}

fr fr Table formatting (simplified)
slay SpillTable(headers tea, rows tea) tea {
    sus tableHeader tea = "| " + headers + " |"
    vibez.spill(tableHeader)
    vibez.spill("|----|")
    sus tableRows tea = "| " + rows + " |"
    vibez.spill(tableRows)
    damn tableHeader
}

fr fr JSON formatting (simplified)
slay SpillJSON(data tea) tea {
    sus jsonOutput tea = "{\"data\": \"" + data + "\"}"
    vibez.spill(jsonOutput)
    damn jsonOutput
}

fr fr List formatting
slay SpillList(items tea) tea {
    sus listOutput tea = "- " + items
    vibez.spill(listOutput)
    damn listOutput
}

fr fr Progress bar (simplified implementation)
slay NewProgressBar(total normie) tea {
    sus progressBar tea = "Progress: [" + "=" + "] 0/" + "100"
    damn progressBar
}

fr fr Spinner (simplified implementation)
slay NewSpinner() tea {
    sus spinner tea = "Loading..."
    damn spinner
}

fr fr GenZ formatting functions
slay ConvertToGenZ(text tea) tea {
    sus genZText tea = text + " fr fr"
    damn genZText
}

slay SpillGenZ(message tea) tea {
    sus genZMessage tea = ConvertToGenZ(message)
    vibez.spill(genZMessage)
    damn genZMessage
}

slay FormatNumGenZ(num normie) tea {
    sus formattedNum tea = "42K"
    damn formattedNum
}

slay SpillWithEmojis(message tea) tea {
    sus emojiMessage tea = message + " 🔥"
    vibez.spill(emojiMessage)
    damn emojiMessage
}

slay SetDefaultGenZFormat(format tea) {
    defaultGenZFormat = format
}

fr fr Enhanced formatting functions
slay SpillPretty(data tea) tea {
    sus prettyData tea = "Pretty: " + data
    vibez.spill(prettyData)
    damn prettyData
}

slay GetFactsPretty(data tea) tea {
    sus prettyData tea = "Pretty: " + data
    damn prettyData
}

fr fr Tree formatting (simplified)
slay SpillTree(root tea, branches tea) tea {
    sus treeOutput tea = root + "\n├── " + branches
    vibez.spill(treeOutput)
    damn treeOutput
}

fr fr Map formatting (simplified)
slay SpillMap(data tea) tea {
    sus mapOutput tea = "Map: " + data
    vibez.spill(mapOutput)
    damn mapOutput
}
