#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Please specify a file to read found layouts from and a name to publish as (and potentially a URL to publish to)!"
    exit 1
fi

if [ -z "$2" ]
  then
    echo "Please specify a name to publish as (and potentially a URL to publish to)!"
    exit 1
fi

if [ -z "$3" ]
  then
    URL="https://keyboard-layout-optimizer.herokuapp.com/api"
else
    URL="$3"
fi


for layout in `cat $1`
do
    echo "Publishing $layout to $URL"
    curl -X 'POST' -d "{\"layout\": \"$layout\", \"published_by\": \"$2\"}" $URL
done
