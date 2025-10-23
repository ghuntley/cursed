package org.cursed

import com.intellij.psi.tree.IElementType
import com.intellij.psi.tree.TokenSet

object CursedTypes {
    val COMMENT = IElementType("COMMENT", CursedLanguage)
    val STRING = IElementType("STRING", CursedLanguage)
    val KEYWORD = IElementType("KEYWORD", CursedLanguage)
    val IDENTIFIER = IElementType("IDENTIFIER", CursedLanguage)

    object Factory {
        fun createElement(node: com.intellij.lang.ASTNode?): com.intellij.psi.PsiElement {
            // TODO: Implement element factory
            return object : com.intellij.psi.impl.PsiElementBase(node) {
                override fun toString(): String = "CursedElement"
            }
        }
    }
}