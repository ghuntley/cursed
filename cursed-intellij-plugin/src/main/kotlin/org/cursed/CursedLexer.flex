package org.cursed;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;
import org.cursed.psi.CursedTypes;

%%

%class _CursedLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType
%eof{  return;
%eof}

%{
  private int commentStart;
  private int commentDepth;
%}

LineTerminator = \r|\n|\r\n
InputCharacter = [^\r\n]
WhiteSpace = {LineTerminator} | [ \t\f]

Comment = {TraditionalComment} | {EndOfLineComment} | {DocumentationComment}

TraditionalComment   = "/*" [^*] ~"*/" | "/*" "*"+ "/"
EndOfLineComment      = "//" {InputCharacter}* {LineTerminator}?
DocumentationComment = "/**" {CommentContent} "*"+ "/"
CommentContent       = ( [^*] | \*+ [^/*] )*

Identifier = [:jletter:] [:jletterdigit:]*

DecIntegerLiteral = 0 | [1-9][0-9]*

%%

<YYINITIAL> {
  /* keywords */
  "vibe"                      { return CursedTypes.VIBE; }
  "yeet"                      { return CursedTypes.YEET; }
  "facts"                     { return CursedTypes.FACTS; }
  "sus"                       { return CursedTypes.SUS; }
  "be_like"                   { return CursedTypes.BE_LIKE; }
  "slay"                      { return CursedTypes.SLAY; }
  "squad"                     { return CursedTypes.SQUAD; }
  "collab"                    { return CursedTypes.COLLAB; }
  "ready"                     { return CursedTypes.READY; }
  "otherwise"                 { return CursedTypes.OTHERWISE; }
  "vibe_check"                { return CursedTypes.VIBE_CHECK; }
  "mood"                      { return CursedTypes.MOOD; }
  "basic"                     { return CursedTypes.BASIC; }
  "bestie"                    { return CursedTypes.BESTIE; }
  "flex"                      { return CursedTypes.FLEX; }
  "periodt"                   { return CursedTypes.PERIODT; }
  "damn"                      { return CursedTypes.DAMN; }
  "ghosted"                   { return CursedTypes.GHOSTED; }
  "simp"                      { return CursedTypes.SIMP; }
  "later"                     { return CursedTypes.LATER; }
  "stan"                      { return CursedTypes.STAN; }
  "yikes"                     { return CursedTypes.YIKES; }
  "fam"                       { return CursedTypes.FAM; }
  "shook"                     { return CursedTypes.SHOOK; }
  "dm_send"                   { return CursedTypes.DM_SEND; }
  "dm_recv"                   { return CursedTypes.DM_RECV; }
  "dm_close"                  { return CursedTypes.DM_CLOSE; }

  /* types */
  "normie"                    { return CursedTypes.TYPE; }
  "smol"                      { return CursedTypes.TYPE; }
  "mid"                       { return CursedTypes.TYPE; }
  "thicc"                     { return CursedTypes.TYPE; }
  "snack"                     { return CursedTypes.TYPE; }
  "meal"                      { return CursedTypes.TYPE; }
  "byte"                      { return CursedTypes.TYPE; }
  "rune"                      { return CursedTypes.TYPE; }
  "sip"                       { return CursedTypes.TYPE; }
  "extra"                     { return CursedTypes.TYPE; }
  "tea"                       { return CursedTypes.TYPE; }
  "lit"                       { return CursedTypes.TYPE; }
  "map"                       { return CursedTypes.TYPE; }
  "dm"                        { return CursedTypes.TYPE; }

  /* literals */
  "based"                     { return CursedTypes.BOOLEAN_LITERAL; }
  "cringe"                    { return CursedTypes.BOOLEAN_LITERAL; }
  "nah"                       { return CursedTypes.NIL_LITERAL; }

  /* separators */
  "("                         { return CursedTypes.LPAREN; }
  ")"                         { return CursedTypes.RPAREN; }
  "{"                         { return CursedTypes.LBRACE; }
  "}"                         { return CursedTypes.RBRACE; }
  "["                         { return CursedTypes.LBRACK; }
  "]"                         { return CursedTypes.RBRACK; }
  ";"                         { return CursedTypes.SEMICOLON; }
  ","                         { return CursedTypes.COMMA; }
  "."                         { return CursedTypes.DOT; }

  /* operators */
  "="                         { return CursedTypes.EQ; }
  ">"                         { return CursedTypes.GT; }
  "<"                         { return CursedTypes.LT; }
  "!"                         { return CursedTypes.NOT; }
  "~"                         { return CursedTypes.TILDE; }
  "?"                         { return CursedTypes.QUESTION; }
  ":"                         { return CursedTypes.COLON; }
  "=="                        { return CursedTypes.EQEQ; }
  "<="                        { return CursedTypes.LTEQ; }
  ">="                        { return CursedTypes.GTEQ; }
  "!="                        { return CursedTypes.NOTEQ; }
  "&&"                        { return CursedTypes.ANDAND; }
  "||"                        { return CursedTypes.OROR; }
  "++"                        { return CursedTypes.PLUSPLUS; }
  "--"                        { return CursedTypes.MINUSMINUS; }
  "+"                         { return CursedTypes.PLUS; }
  "-"                         { return CursedTypes.MINUS; }
  "*"                         { return CursedTypes.STAR; }
  "/"                         { return CursedTypes.DIV; }
  "&"                         { return CursedTypes.AND; }
  "|"                         { return CursedTypes.OR; }
  "^"                         { return CursedTypes.XOR; }
  "%"                         { return CursedTypes.PERCENT; }
  "<<"                        { return CursedTypes.LTLT; }
  ">>"                        { return CursedTypes.GTGT; }
  "&^"                        { return CursedTypes.ANDXOR; }
  "+="                        { return CursedTypes.PLUSEQ; }
  "-="                        { return CursedTypes.MINUSEQ; }
  "*="                        { return CursedTypes.STAREQ; }
  "/="                        { return CursedTypes.DIVEQ; }
  "&="                        { return CursedTypes.ANDEQ; }
  "|="                        { return CursedTypes.OREQ; }
  "^="                        { return CursedTypes.XOREQ; }
  "<<="                       { return CursedTypes.LTLTEQ; }
  ">>="                       { return CursedTypes.GTGTEQ; }
  "&^="                       { return CursedTypes.ANDXOREQ; }
  "%="                        { return CursedTypes.PERCENTEQ; }
  ":="                        { return CursedTypes.COLONEQ; }
  "..."                       { return CursedTypes.ELLIPSIS; }
  "ඞ"                         { return CursedTypes.POINTER; }

  /* string literal */
  \"                          { yybegin(STRING); return CursedTypes.STRING_START; }

  /* character literal */
  \'                          { yybegin(CHAR); return CursedTypes.CHAR_START; }

  /* numeric literals */
  {DecIntegerLiteral}         { return CursedTypes.INT_LITERAL; }

  /* identifiers */
  {Identifier}                { return CursedTypes.IDENTIFIER; }

  /* whitespace */
  {WhiteSpace}                { return com.intellij.psi.TokenType.WHITE_SPACE; }

  /* comments */
  {Comment}                   { return CursedTypes.COMMENT; }
}

<STRING> {
  \"                          { yybegin(YYINITIAL); return CursedTypes.STRING_END; }
  [^\n\r\"\\]+               { return CursedTypes.STRING_CONTENT; }
  \\.                         { return CursedTypes.STRING_ESCAPE; }
}

<CHAR> {
  \'                          { yybegin(YYINITIAL); return CursedTypes.CHAR_END; }
  [^\n\r\'\\]                 { return CursedTypes.CHAR_CONTENT; }
  \\.                         { return CursedTypes.CHAR_ESCAPE; }
}

/* error fallback */
[^]                           { return com.intellij.psi.TokenType.BAD_CHARACTER; }