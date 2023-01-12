import sys
import os

args = sys.argv

if len(args) <= 1:
    print('Must provide a day argument. ex: python new_day.py 2')
    exit()

days = range(1, 25 + 1)

day = int(args[1])

if day not in days:
    print('new day not in range: must be between 1-25')
    exit()


script_path = os.path.dirname(__file__)

path_to_day_program = os.path.join(script_path, 'src', 'bin', 'day_' + args[1] + '.rs')

if os.path.exists(path_to_day_program):
    print('advent of code day ' + str(day) +  ' has already been created')
    exit()

# create new ./inputs/day_#/example.txt file
new_day_inputs_path = os.path.join(script_path, 'inputs', 'day_' + args[1])

if not os.path.exists(new_day_inputs_path):
    os.makedirs(new_day_inputs_path)

new_inputs_example_file = os.path.join(new_day_inputs_path, 'example.txt')

with open(new_inputs_example_file, 'a'):
    print('./inputs/day_' + args[1] + '/example.txt created!')

# create new ./src/bin/day_#.rs file
new_day_program_path = os.path.join(script_path, 'src', 'bin')

if not os.path.exists(new_day_program_path):
    os.makedirs(new_day_program_path)

new_day_program = os.path.join(new_day_program_path, 'day_' + args[1] + '.rs')

with open(new_day_program, 'a') as day_program:
    day_program.write('fn main() {')
    day_program.write('\n')
    day_program.write('\ttodo!();')
    day_program.write('\n')
    day_program.write('}')
    day_program.write('\n')
    print('./src/bin/day_' + args[1] + '.rs created!')