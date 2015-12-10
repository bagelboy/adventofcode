inputs = {}
outputs = {}

def evaluate(wire):
    if wire.isnumeric():
        result = int(wire)
    elif wire in outputs:
        result = outputs[wire]
    else:
        instr = inputs[wire]
        if len(instr) == 1:
            in_value = instr[0]
            if in_value.isnumeric():
                result = int(in_value)
            result = evaluate(in_value)
        elif len(instr) == 2:
            op = instr[0]
            arg = instr[1]
            if op == 'NOT':
                result = ~evaluate(arg)
            else:
                raise NotImplementedError('unexpected op:', op)
        else:
            op = instr[1]
            left = evaluate(instr[0])
            right = evaluate(instr[2])
            if op == 'AND':
                result = left & right
            elif op == 'OR':
                result = left | right
            elif op == 'LSHIFT':
                result = left << right
            elif op == 'RSHIFT':
                result = left >> right
            else:
                raise NotImplementedError('unexpected op:', op)
    outputs[wire] = result
    return result

with open('input') as input_data:
    for line in input_data:
        conn_in, conn_out = line.rstrip().split(' -> ')
        conn_parts = tuple(conn_in.split())
        inputs[conn_out] = conn_parts

signal_a = evaluate('a')
print(signal_a)

outputs = {'b': signal_a}
print(evaluate('a'))
