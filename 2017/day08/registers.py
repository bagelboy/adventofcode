from collections import defaultdict


def process_instruction(instruction, registers):
    dest, op, amount, _, lhs, condition, rhs = instruction.split()
    lhs = registers[lhs]
    rhs = int(rhs)

    condition_satisfied = (condition == '>' and lhs > rhs) or (
        condition == '<'
        and lhs < rhs) or (condition == '<=' and lhs <= rhs) or (
            condition == '>='
            and lhs >= rhs) or (condition == '=='
                                and lhs == rhs) or (condition == '!='
                                                    and lhs != rhs)
    if condition_satisfied:
        if op == 'inc':
            registers[dest] += int(amount)
        else:
            registers[dest] -= int(amount)


if __name__ == '__main__':
    registers = defaultdict(int)
    overall_max = 0
    with open('input') as input:
        for line in input:
            process_instruction(line.rstrip(), registers)
            current_max = max(registers.values())
            overall_max = max(overall_max, max(registers.values()))
    print('largest register value:', max(registers.values()))
    print('largest value overall:', overall_max)
