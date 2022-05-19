# PChain

PChain is a command program that chains multiple executions of programs in parallel passing inputs and outputs between then as configured in a PCH file.

## PCH Setup Specification

```pch
[program:alias*times]
>argument1 argument2 $alias.method:origin ...
>...
|input line 1
|input line 2 $alias.method:origin ...
|...
```

### `program`

Is the name of the program the value(s) will be coming and will be passed to.

### `:alias`

Is the alias for the name of the program the methods will call. Default is program.

### `*times`

Is the number of parallel executions of that program will be started. Default is 1.

### `$alias`

Is the name of the alias of the program the value(s) will be coming for this one.

### `.method`

Is the way the value(s) will be passed to this program. Default is `all`.

Options are:

- `all`
  > groups in one line the whole output.
- `each`
  > pass line by line the expected output.
- `fork`
  > distributes the lines for the parallels.
- `nth`
  > gets the line of the specified number.

### `:origin`

Is from what source the value(s) will be coming. Default is `out`.

Options are:

- `out`
  > comes from stdout.
- `err`
  > comes from stderr.

## Examples

```pch
[prog1]
>arg1 arg2 "arg with space"
```

This ICH setup starts prog1 and pass three arguments arg1, arg2 and "arg with space".

```pch
[prog1]
>--input file.txt

[prog2]
|$prog1
```

This ICH setup starts prog1 and pass two arguments --input and file.txt. In parallel the prog2 will be started and will wait to prog1 to end and will pass all the output of prog1 into the input of prog2.
