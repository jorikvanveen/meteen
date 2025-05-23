#!/bin/bash
rm -r bindings src/entity;
sea-orm-cli generate entity \
  --database-url $DATABASE_URL \
  --with-serde both \
  --model-extra-derives "ts_rs::TS" \
  --model-extra-attributes "ts(export),ts(export_to = \"{table_name}Model.ts\")" \
  -o src/entity/ &&

for filename in src/entity/*; do
  table_name=$(cat $filename | sed -En 's/#\[sea_orm\(table_name = "(.*)"\)\]/\1/p');

  if [[ $table_name != "" ]]; then
    sed -i "s/{table_name}/${table_name^}/" $filename;
  fi;
done &&

cargo test export_bindings
