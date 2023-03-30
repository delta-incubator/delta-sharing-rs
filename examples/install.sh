#!/usr/bin/env bash
if ! pyenv versions | grep -q 3.8.13; then
    pyenv install 3.8.13
fi

if ! pyenv versions | grep -q spark3; then
    pyenv virtualenv 3.8.13 spark3
fi

pyenv local spark3

if ! command -v pyspark 1>/dev/null 2>&1 ; then
    pip install pyspark delta-spark
fi

PYSPARK_LOCATION=$(pip show pyspark | grep -e "Location" | cut -d ' ' -f 2)
PYSPARK_NAME=$(pip show pyspark | grep -e "Name" | cut -d ' ' -f 2)

export SPARK_HOME="$PYSPARK_LOCATION/$PYSPARK_NAME"
export PYSPARK_PYTHON=$(pyenv which python)

#pyspark \
#    --packages io.delta:delta-core_2.12:2.2.0 \
#    --conf "spark.sql.extensions=io.delta.sql.DeltaSparkSessionExtension" \
#    --conf "spark.sql.catalog.spark_catalog=org.apache.spark.sql.delta.catalog.DeltaCatalog"
