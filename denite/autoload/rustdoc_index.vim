let s:save_cpo = &cpo
set cpo&vim

" This function is Copyright 2015 rhysd http://opensource.org/licenses/MIT
function! s:open_url(url) abort
    let url = shellescape(a:url)
    if has('win32') || has('win64')
        let cmd = 'rundll32 url.dll,FileProtocolHandler ' . url
    elseif executable('xdg-open') && has('unix')
        let cmd = 'xdg-open ' . url
    elseif executable('open') && has('mac')
        let cmd = 'open ' . url
    elseif executable('google-chrome')
        let cmd = 'google-chrome ' . url
    elseif executable('firefox')
        let cmd = 'firefox ' . url
    else
        call s:error('No command is found to open URL. Please set g:rust_doc#open_cmd')
        return
    endif

    let output = system(cmd)
    if v:shell_error
        call s:error('Failed to open ' . a:url . ': ' . output)
        return
    endif
endfunction

function! s:error(msg) abort
    echohl Error
    echomsg 'rustdoc-index: ' . a:msg
    echohl None
endfunction

function! rustdoc_index#open_denite(url) abort
    try
        call openbrowser#open(a:url)
    catch /^Vim\%((\a\+)\)\=:E117/
        call s:open_url(a:url)
    endtry
endfunction

let &cpo = s:save_cpo
unlet s:save_cpo
