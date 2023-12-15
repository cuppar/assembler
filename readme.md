# Hack Assembler

Hack assembler process the output from Jack compiler backend [`vmtranslator`](https://github.com/cuppar/vmtranslator).

- `assembler` transfer a `XXX.asm` assembly program to a `XXX.hack` file.
- The `XXX.hack` file is the `Hack` machine code file, it can be execution by `Hack` machine(A simple von Neumann machine).

## Example

```bash
$ assembler Max.asm
```

### Assembly code

Max.asm

```
function Main.fibonacci 0
	push argument 0
	push constant 2
	lt                     
	if-goto N_LT_2        
	goto N_GE_2
label N_LT_2               // if n < 2 returns n
	push argument 0        
	return
label N_GE_2               // if n >= 2 returns fib(n - 2) + fib(n - 1)
	push argument 0
	push constant 2
	sub
	call Main.fibonacci 1  // computes fib(n - 2)
	push argument 0
	push constant 1
	sub
	call Main.fibonacci 1  // computes fib(n - 1)
	add                    // returns fib(n - 1) + fib(n - 2)
	return
```

### Machine code

Max.hack

```
0000000000000000
1111110000010000
0000000000000001
1111010011010000
0000000000001010
1110001100000001
0000000000000001
1111110000010000
0000000000001100
1110101010000111
0000000000000000
1111110000010000
0000000000000010
1110001100001000
0000000000001110
1110101010000111
```