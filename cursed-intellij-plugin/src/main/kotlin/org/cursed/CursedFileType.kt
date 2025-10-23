package org.cursed

import com.intellij.openapi.fileTypes.LanguageFileType
import javax.swing.Icon

object CursedFileType : LanguageFileType(CursedLanguage) {
    override fun getName(): String = "CURSED"

    override fun getDescription(): String = "CURSED programming language"

    override fun getDefaultExtension(): String = "cursed"

    override fun getIcon(): Icon? = null
}