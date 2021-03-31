if (exists('g:loaded_rustdoc_index') && g:loaded_rustdoc_index) || &cp
    finish
endif

echo "asdf"

let g:loaded_rustdoc_index = 1
