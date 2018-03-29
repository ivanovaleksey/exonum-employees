#!/bin/bash

function print_help {
    echo "Usage:
    --all             Display info for all employees
    --key {key}       Display info for employee with key: {key}
    --blocks {key}    Display info for employee blocks"
}

if [[ $1 == "--all" ]];
then
    curl http://127.0.0.1:8000/api/services/employees/employees
elif [[ $1 == "--key" ]];
then
    curl http://127.0.0.1:8000/api/services/employees/employees/$2
elif [[ $1 == "--blocks" ]];
then
    curl http://127.0.0.1:8000/api/services/employees/employees/$2/blocks
else
    print_help
    exit
fi
