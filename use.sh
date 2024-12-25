# ./add_rs consumer_product_goods.rs orderNum
for i in consumer_product_goods.rs; do
    i=$(realpath $i)
    if ! grep -q "sea_orm(column_name" $i; then
        continue
    fi
    echo $i
    fields=$(cat $i | grep "sea_orm(column_name"| awk '{print $3}' | cut -d ')' -f1 | sed 's/"//g')
    for j in $fields; do
        ./add_rs $i $j
    done
done
