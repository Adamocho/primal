" Vim syntax file
" Language: Primal Language
" Maintainer: Adamocho
" Latest Revision: 15 November 2024

if exists("b:current_syntax")
  finish
endif


" Comments
syn match comment "#.*$"

" Integer with - + or nothing in front
syn match number '\d\+'
syn match number '[-+]\d\+'

" Strings
syn region string start='"' end='"'

" Keywords
syn keyword basicKeywords LET PRINT INPUT
syn keyword truthValues true false
syn keyword condKeywords AND NOT OR
syn keyword ifKeywords IF ENDIF THEN
syn keyword whileKeywords WHILE DO ENDWHILE

" Highlighting
hi def link comment         Comment
hi def link basicKeywords   Statement
hi def link condKeywords    Label
hi def link string          String
hi def link number          Number
hi def link truthValues     Boolean
hi def link ifKeywords      Conditional 
hi def link whileKeywords   Repeat


" Setting the file type
au BufRead,BufNewFile *.roq setfiletype pml
let b:current_syntax = "pml"
