# Macros
Fish&C currenllt implements 2 macros, `define` and `import`. macros are defined in a header which is declared in the following way: 
```Fish&C
#HEADER
    [macros]
#HEADER
```

There can be multiple macros in one header.

## Define
```Fish&C
    define ARG1 ARG2
```
changes all apearances of the `ARG2` in the code to `ARG1`.

## Import
```Fish&C
    define name.fac
```
Imports the code of the file imported by appending the code to the end of the original code.