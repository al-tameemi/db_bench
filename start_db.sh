#!/run/current-system/sw/bin/bash
influxdb3 serve --object-store file --data-dir ./.influxdb3 --node-id test_node &
docker run -p 9000:9000 -p 9009:9009 -p 8812:8812 -p 9003:9003 questdb/questdb:9.0.0 &