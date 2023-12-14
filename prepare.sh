#!/bin/bash

BASE=${1:-database}

if [ -e EPSG*PostgreSQL.zip ]; then
  if [ ! -d ${BASE} ]; then
    mkdir ${BASE}
  fi

  cd ${BASE} && cp ../EPSG*PostgreSQL.zip . && unzip EPSG*PostgreSQL.zip
  mv *Table_Script.sql 01.sql
  mv *Data_Script.sql  02.sql
  mv *FKey_Script.sql  03.sql
else 
  echo "No zip file found. Looking for EPSG*PostgreSQL.zip"
  exit 255
fi
