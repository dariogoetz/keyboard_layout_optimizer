#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Please specify a file to read found layouts from and a name to publish as (and potentially a layout config name and a URL to publish to)!"
    exit 1
fi

if [ -z "$2" ]
  then
    echo "Please specify a name to publish as (and a layout config name potentially a URL to publish to)!"
    exit 1
fi

if [ -z "$3" ]
  then
    LAYOUT_CONFIG="standard"
else
    LAYOUT_CONFIG="$3"
fi

if [ -z "$4" ]
  then
    URL="https://keyboard-layout-optimizer.fly.dev/api"
else
    URL="$4"
fi


for LAYOUT in `cat $1`
do
    echo "Publishing $LAYOUT to $URL"
    curl -X 'POST' -d "{\"layout\": \"$LAYOUT\", \"published_by\": \"$2\", \"layout_config\": \"$LAYOUT_CONFIG\"}" $URL
done
