#!/bin/sh

echo Starting basiliq on port $PORT

/app/basiliq --demo serve -N 19 -P $PORT -H 0.0.0.0 --dynamic-config
