package org.cursed

import com.intellij.lexer.FlexAdapter
import com.intellij.lexer.LexerBase
import com.intellij.psi.tree.IElementType

class CursedLexer : FlexAdapter(_CursedLexer(null))