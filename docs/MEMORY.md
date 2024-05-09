# Memory variables

## Memory management
There is a continuous line of memory of size `n` named `mem`, thats where all the data is stored.

`mem_fi` is the possition of the last allocated part of memory.

## Printing
`p_buf` is a segment of memory where the data that is to be printed is buffered.

`p_buf_ptr` is stores the last writen possition of the buffer.

## Functions

### Function registers
`rdi` manages the acces to mem.

`r15` stores the start possition of the memory in the current function (not needed)


