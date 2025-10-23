package org.cursed

import com.intellij.lang.ASTNode
import com.intellij.lang.ParserDefinition
import com.intellij.lang.PsiParser
import com.intellij.lexer.Lexer
import com.intellij.openapi.project.Project
import com.intellij.psi.FileViewProvider
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.psi.tree.IFileElementType
import com.intellij.psi.tree.TokenSet

class CursedParserDefinition : ParserDefinition {
    override fun createLexer(project: Project?): Lexer = CursedLexer()

    override fun createParser(project: Project?): PsiParser = CursedParser()

    override fun getFileNodeType(): IFileElementType = FILE

    override fun getCommentTokens(): TokenSet = TokenSet.create(CursedTypes.COMMENT)

    override fun getStringLiteralElements(): TokenSet = TokenSet.create(CursedTypes.STRING)

    override fun createElement(node: ASTNode?): PsiElement = CursedTypes.Factory.createElement(node)

    override fun createFile(viewProvider: FileViewProvider): PsiFile = CursedFile(viewProvider)

    companion object {
        val FILE = IFileElementType(CursedLanguage)
    }
}