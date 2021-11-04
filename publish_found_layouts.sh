#!/usr/bin/env bash

if [ -z "$2" ]
  then
    echo "Please specify a name to publish as!"
    exit 1
fi

if [ -z "$3" ]
  then
    URL="http://localhost:8000"
else
    URL="$3"
fi


for layout in `cat $1`
do
    echo "Publishing $layout to $URL"
    curl -X 'POST' -d "{\"layout\": \"$layout\", \"published_by\": \"$2\"}" $URL
done
