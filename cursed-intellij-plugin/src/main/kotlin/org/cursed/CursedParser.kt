package org.cursed

import com.intellij.lang.PsiBuilder
import com.intellij.lang.PsiParser
import com.intellij.psi.tree.IElementType

class CursedParser : PsiParser {
    override fun parse(root: IElementType, builder: PsiBuilder): com.intellij.lang.ASTNode {
        // TODO: Implement parser
        return builder.treeBuilt
    }
}