#!/bin/sh

echo Starting basiliq on port $PORT

/app/basiliq --dbconn_number 19 serve -P $PORT -H 0.0.0.0 --dynamic-config --demo
