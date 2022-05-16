# IChain

IChain is a command program that chains multiple executions of programs in parallel passing inputs and outputs between then as configured in a ICH file.

## ICH Setup Specification

### Basic

```ich
[name_of_program]
>arguments...
|inputs...
```

### Channeled

```ich
[program]
>arg1 arg2
|input1
|input2

[program_two]
>$program.method:origin
```

This setup will pass to `program_two` as argument the output from `origin` of `program` using the `method` of passing the value(s).

### `program`

Is the name from what program the value(s) will be coming.

### `method`

Is the way the value(s) will be passed. If it is not configured the default is `all`.

Options are:

- `all`
  > groups in one line the whole output.
- `each`
  > pass line by line the expected output.
- `nth`
  > gets the line of the specified number.

### `origin`

Is from what source the value(s) will be coming. If it is not configured the default is `out`.

Options are:

- `out`
  > comes from stdout.
- `err`
  > comes from stderr.

## Examples

```ich
[prog1]
>arg1 arg2 "arg with space"
```

This ICH setup starts

prog1 and pass three arguments arg1, arg2 and "arg with space".

```ich
[prog1]
>--input file.txt

[prog2]
|$prog1
```

This ICH setup starts prog1 and pass two arguments --input and file.txt. In parallel the prog2 will be started and will wait to prog1 to end and will pass all the output of prog1 into the input of prog2.
