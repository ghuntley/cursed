package org.cursed

import com.intellij.lang.ASTNode
import com.intellij.lang.ParserDefinition
import com.intellij.lang.PsiBuilder
import com.intellij.lang.PsiBuilderFactory
import com.intellij.openapi.util.text.StringUtil
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.psi.tree.IElementType
import com.intellij.testFramework.ParsingTestCase
import org.junit.Test
import org.junit.Assert.*

class CursedParserTest : ParsingTestCase("cursed", "cursed", CursedParserDefinition()) {

    override fun getTestDataPath(): String {
        return "testData"
    }

    @Test
    fun testSimplePackage() {
        doTest("vibe main", """
            CURSED_FILE
              PACKAGE_CLAUSE
                VIBE_KEYWORD
                IDENTIFIER
        """.trimIndent())
    }

    @Test
    fun testFunctionDeclaration() {
        doTest("slay main() { }", """
            CURSED_FILE
              FUNCTION_DECLARATION
                SLAY_KEYWORD
                IDENTIFIER
                PARAMETERS
                  LPAREN
                  RPAREN
                BLOCK
                  LBRACE
                  RBRACE
        """.trimIndent())
    }

    @Test
    fun testVariableDeclaration() {
        doTest("sus x normie = 42", """
            CURSED_FILE
              VAR_DECLARATION
                SUS_KEYWORD
                VAR_SPEC
                  IDENTIFIER_LIST
                    IDENTIFIER
                  TYPE
                    TYPE_NAME
                      BUILTIN_TYPE
                  EXPRESSION_LIST
                    EXPRESSION
                      UNARY_EXPRESSION
                        PRIMARY_EXPRESSION
                          OPERAND
                            LITERAL
                              BASIC_LITERAL
                                INT_LITERAL
        """.trimIndent())
    }

    @Test
    fun testTypeDeclaration() {
        doTest("be_like Person squad { name tea }", """
            CURSED_FILE
              TYPE_DECLARATION
                BE_LIKE_KEYWORD
                TYPE_SPEC
                  IDENTIFIER
                  TYPE
                    STRUCT_TYPE
                      SQUAD_KEYWORD
                      LBRACE
                      FIELD_DECLARATION
                        IDENTIFIER_LIST
                          IDENTIFIER
                        TYPE
                          TYPE_NAME
                            BUILTIN_TYPE
                      RBRACE
        """.trimIndent())
    }

    @Test
    fun testIfStatement() {
        doTest("ready x > 0 { damn x }", """
            CURSED_FILE
              IF_STATEMENT
                READY_KEYWORD
                EXPRESSION
                  BINARY_EXPRESSION
                    UNARY_EXPRESSION
                      PRIMARY_EXPRESSION
                        OPERAND
                          IDENTIFIER
                    GT
                    UNARY_EXPRESSION
                      PRIMARY_EXPRESSION
                        OPERAND
                          LITERAL
                            BASIC_LITERAL
                              INT_LITERAL
                BLOCK
                  LBRACE
                  RETURN_STATEMENT
                    DAMN_KEYWORD
                    EXPRESSION_LIST
                      EXPRESSION
                        UNARY_EXPRESSION
                          PRIMARY_EXPRESSION
                            OPERAND
                              IDENTIFIER
                  RBRACE
        """.trimIndent())
    }

    @Test
    fun testForLoop() {
        doTest("bestie i normie = 0; i < 10; i++ { }", """
            CURSED_FILE
              FOR_STATEMENT
                BESTIE_KEYWORD
                FOR_CLAUSE
                  SIMPLE_STATEMENT
                    SHORT_VAR_DECLARATION
                      IDENTIFIER_LIST
                        IDENTIFIER
                      COLONEQ
                      EXPRESSION_LIST
                        EXPRESSION
                          UNARY_EXPRESSION
                            PRIMARY_EXPRESSION
                              OPERAND
                                LITERAL
                                  BASIC_LITERAL
                                    INT_LITERAL
                  SEMICOLON
                  EXPRESSION
                    BINARY_EXPRESSION
                      UNARY_EXPRESSION
                        PRIMARY_EXPRESSION
                          OPERAND
                            IDENTIFIER
                      LT
                      UNARY_EXPRESSION
                        PRIMARY_EXPRESSION
                          OPERAND
                            LITERAL
                              BASIC_LITERAL
                                INT_LITERAL
                  SEMICOLON
                  INC_DEC_STATEMENT
                    IDENTIFIER
                    PLUSPLUS
                BLOCK
                  LBRACE
                  RBRACE
        """.trimIndent())
    }

    @Test
    fun testErrorHandling() {
        doTest("yikes \"error message\"", """
            CURSED_FILE
              ERROR_STATEMENT
                YIKES_KEYWORD
                EXPRESSION
                  UNARY_EXPRESSION
                    PRIMARY_EXPRESSION
                      OPERAND
                        LITERAL
                          BASIC_LITERAL
                            STRING_LITERAL
        """.trimIndent())
    }

    @Test
    fun testRecoveryStatement() {
        doTest("fam { } sus err { }", """
            CURSED_FILE
              RECOVERY_STATEMENT
                FAM_KEYWORD
                BLOCK
                  LBRACE
                  RBRACE
                IDENTIFIER
                BLOCK
                  LBRACE
                  RBRACE
        """.trimIndent())
    }

    @Test
    fun testChannelOperations() {
        doTest("dm_send(ch, value)", """
            CURSED_FILE
              SEND_STATEMENT
                DM_SEND_KEYWORD
                LPAREN
                EXPRESSION
                  UNARY_EXPRESSION
                    PRIMARY_EXPRESSION
                      OPERAND
                        IDENTIFIER
                COMMA
                EXPRESSION
                  UNARY_EXPRESSION
                    PRIMARY_EXPRESSION
                      OPERAND
                        IDENTIFIER
                RPAREN
        """.trimIndent())
    }

    @Test
    fun testReceiveStatement() {
        doTest("sus msg = dm_recv(ch)", """
            CURSED_FILE
              VAR_DECLARATION
                SUS_KEYWORD
                VAR_SPEC
                  IDENTIFIER_LIST
                    IDENTIFIER
                  EXPRESSION_LIST
                    EXPRESSION
                      UNARY_EXPRESSION
                        PRIMARY_EXPRESSION
                          RECEIVE_STATEMENT
                            DM_RECV_KEYWORD
                            LPAREN
                            EXPRESSION
                              UNARY_EXPRESSION
                                PRIMARY_EXPRESSION
                                  OPERAND
                                    IDENTIFIER
                            RPAREN
        """.trimIndent())
    }

    @Test
    fun testComments() {
        doTest("fr fr line comment", """
            CURSED_FILE
              COMMENT
        """.trimIndent())
    }

    @Test
    fun testBlockComment() {
        val input = """
            no cap
            block comment
            on god
        """.trimIndent()

        doTest(input, """
            CURSED_FILE
              COMMENT
        """.trimIndent())
    }

    @Test
    fun testComplexProgram() {
        val input = """
            vibe main

            slay main() {
                sus x normie = 42
                ready x > 0 {
                    vibez.spill("positive")
                }
            }
        """.trimIndent()

        doTest(input, """
            CURSED_FILE
              PACKAGE_CLAUSE
                VIBE_KEYWORD
                IDENTIFIER
              FUNCTION_DECLARATION
                SLAY_KEYWORD
                IDENTIFIER
                PARAMETERS
                  LPAREN
                  RPAREN
                BLOCK
                  LBRACE
                  VAR_DECLARATION
                    SUS_KEYWORD
                    VAR_SPEC
                      IDENTIFIER_LIST
                        IDENTIFIER
                      TYPE
                        TYPE_NAME
                          BUILTIN_TYPE
                      EXPRESSION_LIST
                        EXPRESSION
                          UNARY_EXPRESSION
                            PRIMARY_EXPRESSION
                              OPERAND
                                LITERAL
                                  BASIC_LITERAL
                                    INT_LITERAL
                  IF_STATEMENT
                    READY_KEYWORD
                    EXPRESSION
                      BINARY_EXPRESSION
                        UNARY_EXPRESSION
                          PRIMARY_EXPRESSION
                            OPERAND
                              IDENTIFIER
                        GT
                        UNARY_EXPRESSION
                          PRIMARY_EXPRESSION
                            OPERAND
                              LITERAL
                                BASIC_LITERAL
                                  INT_LITERAL
                    BLOCK
                      LBRACE
                      EXPRESSION_STATEMENT
                        EXPRESSION
                          UNARY_EXPRESSION
                            PRIMARY_EXPRESSION
                              CALL_EXPRESSION
                                PRIMARY_EXPRESSION
                                  SELECTOR_EXPRESSION
                                    OPERAND
                                      IDENTIFIER
                                    DOT
                                    IDENTIFIER
                                ARGUMENT_LIST
                                  LPAREN
                                  EXPRESSION_LIST
                                    EXPRESSION
                                      UNARY_EXPRESSION
                                        PRIMARY_EXPRESSION
                                          OPERAND
                                            LITERAL
                                              BASIC_LITERAL
                                                STRING_LITERAL
                                  RPAREN
                      RBRACE
                  RBRACE
        """.trimIndent())
    }

    private fun doTest(input: String, expectedTree: String) {
        val psiFile = parseText(input)
        val actualTree = toParseTreeText(psiFile)
        assertEquals(expectedTree, actualTree)
    }

    private fun parseText(text: String): PsiFile {
        val project = myProject
        val parserDefinition: ParserDefinition = CursedParserDefinition()
        val lexer = parserDefinition.createLexer(project)
        val parser = parserDefinition.createParser(project)
        val builder = PsiBuilderFactory.getInstance().createBuilder(
            parserDefinition, lexer, text
        )

        return parser.parse(parserDefinition.fileNodeType, builder)
    }

    private fun toParseTreeText(psiFile: PsiFile): String {
        return toParseTreeText(psiFile.node, 0)
    }

    private fun toParseTreeText(node: ASTNode, depth: Int): String {
        val indent = "  ".repeat(depth)
        val typeName = node.elementType.toString().replace("_", "").toUpperCase()
        var result = indent + typeName

        if (node.getChildren(null).isNotEmpty()) {
            result += "\n"
            for (child in node.getChildren(null)) {
                result += toParseTreeText(child, depth + 1) + "\n"
            }
            result = result.trimEnd()
        }

        return result
    }
}