#!/usr/bin/env bash

if [ -z "$1" ]
  then
    URL="https://keyboard-layout-optimizer.fly.dev/api"
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
curl -X 'POST' -d "{\"layout\": \"k.o,yvgclfzß haeiudtrns xqäüöbpwmj\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL


curl -X 'POST' -d "{\"layout\": \"ßqwertzuiopü asdfghjklö yxcvbnm,.ä\", \"layout_config\": \"crkbd\", \"published_by\": \"qwertz\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßxvlcwkhgfqy uiaeosnrtd üöäpzbm,.j\", \"layout_config\": \"crkbd\", \"published_by\": \"neo2\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjduaxphlmwq ctieobnrsg fvüäöyz,.k\", \"layout_config\": \"crkbd\", \"published_by\": \"bone\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluajwbdgyq crieomntsh vxüäöpf,.k\", \"layout_config\": \"crkbd\", \"published_by\": \"mine\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjluaqwbdgyz crieomntsh vxüäöpf,.k\", \"layout_config\": \"crkbd\", \"published_by\": \"mine-A\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluaqwbdgyj crieomntsh vxüäöpf,.k\", \"layout_config\": \"crkbd\", \"published_by\": \"mine-B\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßkuü.ävgcljf hieaodtrns xyö,qbpwmz\", \"layout_config\": \"crkbd\", \"published_by\": \"AdNW\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßk.o,yvgclfz haeiudtrns xqäüöbpwmj\", \"layout_config\": \"crkbd\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL


curl -X 'POST' -d "{\"layout\": \"ßqwertzuiopü asdfghjklö yxcvbnm,.ä\", \"layout_config\": \"ortho\", \"published_by\": \"qwertz\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßxvlcwkhgfqy uiaeosnrtd üöäpzbm,.j\", \"layout_config\": \"ortho\", \"published_by\": \"neo2\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjduaxphlmwq ctieobnrsg fvüäöyz,.k\", \"layout_config\": \"ortho\", \"published_by\": \"bone\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluajwbdgyq crieomntsh vxüäöpf,.k\", \"layout_config\": \"ortho\", \"published_by\": \"mine\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjluaqwbdgyz crieomntsh vxüäöpf,.k\", \"layout_config\": \"ortho\", \"published_by\": \"mine-A\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluaqwbdgyj crieomntsh vxüäöpf,.k\", \"layout_config\": \"ortho\", \"published_by\": \"mine-B\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßkuü.ävgcljf hieaodtrns xyö,qbpwmz\", \"layout_config\": \"ortho\", \"published_by\": \"AdNW\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßk.o,yvgclfz haeiudtrns xqäüöbpwmj\", \"layout_config\": \"ortho\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL


curl -X 'POST' -d "{\"layout\": \"qwertzuiopü asdfghjklöß yxcvbnm,.ä\", \"layout_config\": \"moonlander\", \"published_by\": \"qwertz\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"xvlcwkhgfqy uiaeosnrtdß üöäpzbm,.j\", \"layout_config\": \"moonlander\", \"published_by\": \"neo2\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"jduaxphlmwq ctieobnrsgß fvüäöyz,.k\", \"layout_config\": \"moonlander\", \"published_by\": \"bone\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"zluajwbdgyq crieomntshß vxüäöpf,.k\", \"layout_config\": \"moonlander\", \"published_by\": \"mine\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"jluaqwbdgyz crieomntshß vxüäöpf,.k\", \"layout_config\": \"moonlander\", \"published_by\": \"mine-A\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"zluaqwbdgyj crieomntshß vxüäöpf,.k\", \"layout_config\": \"moonlander\", \"published_by\": \"mine-B\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"kuü.ävgcljf hieaodtrnsß xyö,qbpwmz\", \"layout_config\": \"moonlander\", \"published_by\": \"AdNW\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"k.o,yvgclfz haeiudtrnsß xqäüöbpwmj\", \"layout_config\": \"moonlander\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL

curl -X 'POST' -d "{\"layout\": \"ßqwertzuiopü asdfghjklö yxcvbnm,.ä\", \"layout_config\": \"lily58\", \"published_by\": \"qwertz\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßxvlcwkhgfqy uiaeosnrtd üöäpzbm,.j\", \"layout_config\": \"lily58\", \"published_by\": \"neo2\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjduaxphlmwq ctieobnrsg fvüäöyz,.k\", \"layout_config\": \"lily58\", \"published_by\": \"bone\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluajwbdgyq crieomntsh vxüäöpf,.k\", \"layout_config\": \"lily58\", \"published_by\": \"mine\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßjluaqwbdgyz crieomntsh vxüäöpf,.k\", \"layout_config\": \"lily58\", \"published_by\": \"mine-A\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßzluaqwbdgyj crieomntsh vxüäöpf,.k\", \"layout_config\": \"lily58\", \"published_by\": \"mine-B\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßkuü.ävgcljf hieaodtrns xyö,qbpwmz\", \"layout_config\": \"lily58\", \"published_by\": \"AdNW\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL
curl -X 'POST' -d "{\"layout\": \"ßk.o,yvgclfz haeiudtrns xqäüöbpwmj\", \"layout_config\": \"lily58\", \"published_by\": \"KOY\", \"highlight\": true, \"secret\":\"$SECRET\"}" $URL

