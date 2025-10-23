" CURSED Vim Syntax Highlighting Test Suite
" This file contains tests for CURSED syntax highlighting in Vim

function! TestSyntaxHighlighting()
    " Create a test buffer with CURSED code
    new
    setlocal filetype=cursed

    " Insert test CURSED code
    call append(0, [
        \ 'vibe main',
        \ '',
        \ 'yeet "vibez"',
        \ '',
        \ 'fr fr This is a line comment',
        \ 'no cap',
        \ 'This is a block comment',
        \ 'on god',
        \ '',
        \ 'be_like Person squad {',
        \ '    name tea',
        \ '    age normie',
        \ '    vibes []tea',
        \ '}',
        \ '',
        \ 'slay main() {',
        \ '    sus p Person = Person{',
        \ '        name: "Alice",',
        \ '        age: 30,',
        \ '        vibes: []tea{"coding", "debugging"}',
        \ '    }',
        \ '',
        \ '    ready p.age > 18 {',
        \ '        vibez.spill("Adult:", p.name)',
        \ '    } otherwise ready p.age > 13 {',
        \ '        vibez.spill("Teen:", p.name)',
        \ '    } otherwise {',
        \ '        vibez.spill("Child:", p.name)',
        \ '    }',
        \ '',
        \ '    bestie i normie = 0; i < 10; i++ {',
        \ '        vibez.spill("Count:", i)',
        \ '    }',
        \ '',
        \ '    sus ch dm<tea> = make(dm<tea>, 10)',
        \ '    stan slay() {',
        \ '        dm_send(ch, "Hello from goroutine")',
        \ '    }()',
        \ '',
        \ '    sus msg, ok = dm_recv(ch)',
        \ '    ready ok {',
        \ '        vibez.spill("Received:", msg)',
        \ '    }',
        \ '',
        \ '    dm_close(ch)',
        \ '',
        \ '    fam {',
        \ '        risky_operation()',
        \ '    } sus err {',
        \ '        vibez.spill("Error:", err.message())',
        \ '    }',
        \ '}',
        \ ])

    " Test syntax highlighting groups
    call s:TestSyntaxGroups()

    " Clean up
    bdelete!
endfunction

function! s:TestSyntaxGroups()
    " Test that various syntax groups are defined
    call assert_true(hlexists('cursedKeyword'), 'cursedKeyword syntax group should exist')
    call assert_true(hlexists('cursedConditional'), 'cursedConditional syntax group should exist')
    call assert_true(hlexists('cursedRepeat'), 'cursedRepeat syntax group should exist')
    call assert_true(hlexists('cursedStatement'), 'cursedStatement syntax group should exist')
    call assert_true(hlexists('cursedException'), 'cursedException syntax group should exist')
    call assert_true(hlexists('cursedType'), 'cursedType syntax group should exist')
    call assert_true(hlexists('cursedBoolean'), 'cursedBoolean syntax group should exist')
    call assert_true(hlexists('cursedConstant'), 'cursedConstant syntax group should exist')
    call assert_true(hlexists('cursedString'), 'cursedString syntax group should exist')
    call assert_true(hlexists('cursedComment'), 'cursedComment syntax group should exist')
    call assert_true(hlexists('cursedFunction'), 'cursedFunction syntax group should exist')
    call assert_true(hlexists('cursedOperator'), 'cursedOperator syntax group should exist')
    call assert_true(hlexists('cursedNumber'), 'cursedNumber syntax group should exist')
endfunction

function! TestPluginCommands()
    " Test that plugin commands are available
    call assert_true(exists(':CursedRun'), 'CursedRun command should exist')
    call assert_true(exists(':CursedBuild'), 'CursedBuild command should exist')
    call assert_true(exists(':CursedTest'), 'CursedTest command should exist')
    call assert_true(exists(':CursedFormat'), 'CursedFormat command should exist')
    call assert_true(exists(':CursedLint'), 'CursedLint command should exist')
endfunction

function! TestFiletypeDetection()
    " Test that .💀 files are detected as cursed filetype
    execute 'edit test.💀'
    call assert_equal(&filetype, 'cursed', '.💀 files should be detected as cursed')
    bdelete!

    execute 'edit test.cursed'
    call assert_equal(&filetype, 'cursed', '.cursed files should be detected as cursed')
    bdelete!
endfunction

function! TestIndentation()
    " Test CURSED-specific indentation
    new
    setlocal filetype=cursed

    call append(0, [
        \ 'slay test() {',
        \ 'ready based {',
        \ 'damn "test"',
        \ '} otherwise {',
        \ 'damn "else"',
        \ '}',
        \ '}'
        \ ])

    " Test indentation
    normal gg=G

    " Check that indentation is correct
    call assert_equal(getline(2), '    ready based {', 'ready should be indented')
    call assert_equal(getline(3), '        damn "test"', 'damn should be indented')
    call assert_equal(getline(4), '    } otherwise {', '} otherwise should be indented')
    call assert_equal(getline(5), '        damn "else"', 'damn in else should be indented')

    bdelete!
endfunction

function! TestCommentStrings()
    " Test that comment strings are set correctly
    new
    setlocal filetype=cursed

    call assert_equal(&commentstring, 'fr\ fr\ %s', 'commentstring should be set for line comments')
    call assert_equal(&comments, 'fr\ fr,no\ cap:on\ god', 'comments should be set for CURSED comments')

    bdelete!
endfunction

" Run all tests
function! RunCursedTests()
    call TestSyntaxHighlighting()
    call TestPluginCommands()
    call TestFiletypeDetection()
    call TestIndentation()
    call TestCommentStrings()

    echo "All CURSED Vim tests passed!"
endfunction

" Auto-run tests if this file is sourced directly
if expand('<sfile>') == expand('<sfile>:p')
    call RunCursedTests()
endif