if exists("b:current_syntax")
    finish
endif

syntax keyword tskKeyword if else while fn break continue return
highlight link tskKeyword Keyword

" Integer with - + or nothing in front
syn match tskNumber '\d\+'
syn match tskNumber '[-+]\d\+'
highlight link tskNumber Constant

syn region tskString start=+"+ end=+"+ skip=+\\"+
syn region tskString start=+'+ end=+'+ skip=+\\'+
highlight link tskString Constant

syn match tskComment ";.*$"
highlight link tskComment Comment

syn region tskWord start="[a-zA-Z_]" end="[^a-zA-Z_]"
highlight link tskWord Identifier

" syn region tskFunc start="fn " hs=e+1 end=" "he=s-1
" highlight link tskFunc Type
syn match tskOp "[!&*+%,./<=>?@\\^`|-]"
highlight link tskOp Operator

syn match tskMove "[\[\]\{}()]"
highlight link tskMove Function

syn region tskPointer start="[*&\\^][a-zA-Z_]" end="[^a-zA-Z_]"me=e-1
highlight link tskPointer Operator

let b:current_syntax = "tsk"
