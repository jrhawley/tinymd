cargo build --release
pushd commonmark-spec/
python3 test/spec_tests.py -p ../target/release/tinymd | grep -P "(^Example \d+|passed)" > ../test/commonmark-spec.txt
popd