#!/bin/bash

function print_help {
    echo "Usage:
        --all           Display info for all employees
        --key {key}     Display info for employee with key: {key}"
}

if [[ $1 == "--all" ]];
then
    curl http://127.0.0.1:8000/api/services/employees/employees
elif [[ $1 == "--key" ]];
then
    curl http://127.0.0.1:8000/api/services/employees/employees/$2
else
    print_help
    exit
fi
