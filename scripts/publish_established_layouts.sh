#!/usr/bin/env bash

if [ -z "$1" ]
  then
    URL="https://keyboard-layout-optimizer.herokuapp.com/api"
else
    URL="$1"
fi


echo "Publishing established layouts to $URL"
curl -X 'POST' -d "{\"layout\": \"qwertzuiopüß asdfghjklö yxcvbnm,.ä\", \"published_by\": \"qwertz\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"xvlcwkhgfqyß uiaeosnrtd üöäpzbm,.j\", \"published_by\": \"neo2\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"jduaxphlmwqß ctieobnrsg fvüäöyz,.k\", \"published_by\": \"bone\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"zluajwbdgyqß crieomntsh vxüäöpf,.k\", \"published_by\": \"mine\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"jluaqwbdgyzß crieomntsh vxüäöpf,.k\", \"published_by\": \"mine-A\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"zluaqwbdgyjß crieomntsh vxüäöpf,.k\", \"published_by\": \"mine-B\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"kuü.ävgcljfß hieaodtrns xyö,qbpwmz\", \"published_by\": \"AdNW\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"k.o,y vgclfzß haeiu dtrns xqäüö bpwmj\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
