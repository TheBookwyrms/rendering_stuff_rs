rustc full_test.rs
if [ "$?" == "0" ]; then
    echo "test"
    ./full_test
else
    echo "compilation failed"
fi