#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "Please specify the layout to publish and a name to publish as (and potentially a URL to publish to)!"
    exit 1
fi

if [ -z "$2" ]
  then
    echo "Please specify a name to publish as (and potentially a URL to publish to)!"
    exit 1
fi

if [ -z "$3" ]
  then
    URL="http://keyboard-layout-optimizer.herokuapp.com/api"
else
    URL="$3"
fi


echo "Publishing $1 to $URL"
curl -X 'POST' -d "{\"layout\": \"$1\", \"published_by\": \"$2\"}" $URL
