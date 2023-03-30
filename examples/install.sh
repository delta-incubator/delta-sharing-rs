#!/usr/bin/env bash
pyenv install 3.8.13
pyenv virtualenv 3.8.13 spark3
pyenv shell spark3
pip install pyspark
MY_PYSPARK_LOCATION=$(pip show pyspark | grep -e "Location" | cut -d ' ' -f 2)
MY_PYSPARK_NAME=$(pip show pyspark | grep -e "Name" | cut -d ' ' -f 2)
export SPARK_HOME="$MY_PYSPARK_LOCATION/$MY_PYSPARK_NAME"
export PYSPARK_PYTHON=$(pyenv which python)
pyenv local spark3
