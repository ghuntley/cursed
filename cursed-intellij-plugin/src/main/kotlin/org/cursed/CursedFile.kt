package org.cursed

import com.intellij.extapi.psi.PsiFileBase
import com.intellij.psi.FileViewProvider

class CursedFile(viewProvider: FileViewProvider) : PsiFileBase(viewProvider, CursedLanguage) {
    override fun getFileType() = CursedFileType

    override fun toString(): String = "CURSED File"
}