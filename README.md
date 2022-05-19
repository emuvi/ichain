# PChain

PChain is a command program that chains multiple executions of programs in parallel passing inputs and outputs between then as configured in a PCH file.

## PCH Setup Specification

```pch
[program:alias*times]
>argument1 argument2 $alias/time.method:origin ...
>...
|input line 1
|input line 2 $alias/time.method:origin ...
|...
```

### `program`

Is the name of the program the value(s) will be coming and also will be passed to.

### `:alias`

Is the alias for the name of the program the methods will call. Default is `program`.

### `*times`

Is the number of parallel executions of that program will be started. Default is 1.

### `$alias`

Is the name of the alias of the program the value(s) will be coming for this one.

### `/time`

Is the index from which time the parallel program was started. Default is 1;

### `.method`

Is the way the value(s) will be passed to this program. Default is `all`.

Options are:

- `all`
  > groups in one line the whole output.
- `each`
  > pass line by line the expected output.
- `fork`
  > distributes the lines for the parallels calling.
- `nth`
  > gets the line of the nth specified number.

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

This ICH setup starts one prog1 and pass three arguments arg1, arg2 and "arg with space".

```pch
[prog1:source]
>--input file.txt

[prog2*4]
>--transform $source.fork
```

This ICH setup starts one prog1 with alias source and pass two arguments --input and file.txt. In parallel four executions of prog2 will be started and they will each one of them ask for the output lines from source. In the fork method, the line that one execution of prog2 gets the others does not. In the other methods, they all gets what they ask for.
