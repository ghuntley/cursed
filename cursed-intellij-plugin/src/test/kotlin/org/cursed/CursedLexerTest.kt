package org.cursed

import com.intellij.psi.tree.IElementType
import org.junit.Test
import org.junit.Assert.*

class CursedLexerTest {

    private fun tokenize(input: String): List<Pair<String, IElementType?>> {
        val lexer = CursedLexer()
        lexer.start(input)
        val tokens = mutableListOf<Pair<String, IElementType?>>()

        var tokenType: IElementType? = lexer.tokenType
        while (tokenType != null) {
            val tokenText = input.substring(lexer.tokenStart, lexer.tokenEnd)
            tokens.add(Pair(tokenText, tokenType))
            lexer.advance()
            tokenType = lexer.tokenType
        }

        return tokens
    }

    @Test
    fun testKeywords() {
        val tokens = tokenize("vibe yeet facts sus be_like slay squad collab ready otherwise")

        val expectedTokens = listOf(
            Pair("vibe", CursedTypes.VIBE),
            Pair("yeet", CursedTypes.YEET),
            Pair("facts", CursedTypes.FACTS),
            Pair("sus", CursedTypes.SUS),
            Pair("be_like", CursedTypes.BE_LIKE),
            Pair("slay", CursedTypes.SLAY),
            Pair("squad", CursedTypes.SQUAD),
            Pair("collab", CursedTypes.COLLAB),
            Pair("ready", CursedTypes.READY),
            Pair("otherwise", CursedTypes.OTHERWISE)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }

    @Test
    fun testTypes() {
        val tokens = tokenize("normie smol mid thicc snack meal byte rune sip extra tea lit")

        val expectedTokens = listOf(
            Pair("normie", CursedTypes.TYPE),
            Pair("smol", CursedTypes.TYPE),
            Pair("mid", CursedTypes.TYPE),
            Pair("thicc", CursedTypes.TYPE),
            Pair("snack", CursedTypes.TYPE),
            Pair("meal", CursedTypes.TYPE),
            Pair("byte", CursedTypes.TYPE),
            Pair("rune", CursedTypes.TYPE),
            Pair("sip", CursedTypes.TYPE),
            Pair("extra", CursedTypes.TYPE),
            Pair("tea", CursedTypes.TYPE),
            Pair("lit", CursedTypes.TYPE)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }

    @Test
    fun testLiterals() {
        val tokens = tokenize("based cringe nah")

        val expectedTokens = listOf(
            Pair("based", CursedTypes.BOOLEAN_LITERAL),
            Pair("cringe", CursedTypes.BOOLEAN_LITERAL),
            Pair("nah", CursedTypes.NIL_LITERAL)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }

    @Test
    fun testOperators() {
        val tokens = tokenize("= == != < <= > >= + - * / % && || ! & | ^ << >> &^ += -= *= /= %= &= |= ^= <<= >>= ++ -- :=")

        // Test a few key operators
        assertTrue("Should contain =", tokens.any { it.first == "=" && it.second == CursedTypes.EQ })
        assertTrue("Should contain ==", tokens.any { it.first == "==" && it.second == CursedTypes.EQEQ })
        assertTrue("Should contain !=", tokens.any { it.first == "!=" && it.second == CursedTypes.NOTEQ })
        assertTrue("Should contain +", tokens.any { it.first == "+" && it.second == CursedTypes.PLUS })
        assertTrue("Should contain -", tokens.any { it.first == "-" && it.second == CursedTypes.MINUS })
        assertTrue("Should contain *", tokens.any { it.first == "*" && it.second == CursedTypes.STAR })
        assertTrue("Should contain /", tokens.any { it.first == "/" && it.second == CursedTypes.DIV })
        assertTrue("Should contain :=", tokens.any { it.first == ":=" && it.second == CursedTypes.COLONEQ })
    }

    @Test
    fun testSeparators() {
        val tokens = tokenize("( ) { } [ ] ; , .")

        val expectedTokens = listOf(
            Pair("(", CursedTypes.LPAREN),
            Pair(")", CursedTypes.RPAREN),
            Pair("{", CursedTypes.LBRACE),
            Pair("}", CursedTypes.RBRACE),
            Pair("[", CursedTypes.LBRACK),
            Pair("]", CursedTypes.RBRACK),
            Pair(";", CursedTypes.SEMICOLON),
            Pair(",", CursedTypes.COMMA),
            Pair(".", CursedTypes.DOT)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }

    @Test
    fun testIdentifiers() {
        val tokens = tokenize("main test_function variable_name")

        val expectedTokens = listOf(
            Pair("main", CursedTypes.IDENTIFIER),
            Pair("test_function", CursedTypes.IDENTIFIER),
            Pair("variable_name", CursedTypes.IDENTIFIER)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }

    @Test
    fun testNumbers() {
        val tokens = tokenize("42 3.14 0xFF 0b1010 077")

        assertTrue("Should contain integer", tokens.any { it.first == "42" && it.second == CursedTypes.INT_LITERAL })
        assertTrue("Should contain float", tokens.any { it.first == "3.14" && it.second == CursedTypes.FLOAT_LITERAL })
        assertTrue("Should contain hex", tokens.any { it.first == "0xFF" && it.second == CursedTypes.INT_LITERAL })
        assertTrue("Should contain binary", tokens.any { it.first == "0b1010" && it.second == CursedTypes.INT_LITERAL })
        assertTrue("Should contain octal", tokens.any { it.first == "077" && it.second == CursedTypes.INT_LITERAL })
    }

    @Test
    fun testStrings() {
        val tokens = tokenize("\"hello world\" `raw string`")

        assertTrue("Should contain string", tokens.any { it.first == "\"hello world\"" && it.second == CursedTypes.STRING_LITERAL })
        assertTrue("Should contain raw string", tokens.any { it.first == "`raw string`" && it.second == CursedTypes.RAW_STRING_LITERAL })
    }

    @Test
    fun testComments() {
        val input = """
            fr fr line comment
            no cap
            block comment
            on god
        """.trimIndent()

        val tokens = tokenize(input)

        // Should contain comment tokens
        assertTrue("Should contain line comment", tokens.any { it.second == CursedTypes.COMMENT })
    }

    @Test
    fun testComplexExpression() {
        val tokens = tokenize("sus x normie = 5 + 3 * 2")

        val expectedSequence = listOf(
            "sus", "x", "normie", "=", "5", "+", "3", "*", "2"
        )

        val actualSequence = tokens.map { it.first }
        assertEquals(expectedSequence, actualSequence)
    }

    @Test
    fun testFunctionDeclaration() {
        val tokens = tokenize("slay main() { }")

        val expectedTokens = listOf(
            Pair("slay", CursedTypes.SLAY),
            Pair("main", CursedTypes.IDENTIFIER),
            Pair("(", CursedTypes.LPAREN),
            Pair(")", CursedTypes.RPAREN),
            Pair("{", CursedTypes.LBRACE),
            Pair("}", CursedTypes.RBRACE)
        )

        assertEquals(expectedTokens.size, tokens.size)
        for (i in expectedTokens.indices) {
            assertEquals("Token $i", expectedTokens[i].first, tokens[i].first)
            assertEquals("Token type $i", expectedTokens[i].second, tokens[i].second)
        }
    }
}